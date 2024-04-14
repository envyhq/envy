/* --------------------------------------------------------------------------------------------
 * Copyright (c) Microsoft Corporation. All rights reserved.
 * Licensed under the MIT License. See License.txt in the project root for license information.
 * ------------------------------------------------------------------------------------------ */

import { workspace, ExtensionContext } from "vscode";
import { connect } from "net";

import {
  LanguageClient,
  ServerOptions,
  type LanguageClientOptions,
  type StreamInfo,
} from "vscode-languageclient/node";

let client: LanguageClient;

const NV_LANGUAGE_SERVER_SOCKET_PATH = "/tmp/nv_language_server.socket";

export function activate(context: ExtensionContext) {
  const serverOptions: ServerOptions = () => {
    const socket = connect({ path: NV_LANGUAGE_SERVER_SOCKET_PATH });
    const result: StreamInfo = {
      writer: socket,
      reader: socket,
    };
    return Promise.resolve(result);
  };

  const clientOptions: LanguageClientOptions = {
    documentSelector: [
      { scheme: "file", language: "plaintext" },
      { scheme: "file", language: "nv" },
    ],
  };

  client = new LanguageClient(
    "nvLanguageServer",
    "nv Language Server",
    serverOptions,
    clientOptions
  );

  client.start();
}

export function deactivate(): Thenable<void> | undefined {
  if (!client) {
    return undefined;
  }

  return client.stop();
}
