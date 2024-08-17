import { connect } from "net";
import {
  LanguageClient,
  ServerOptions,
  type LanguageClientOptions,
  type StreamInfo,
} from "vscode-languageclient/node";

let client: LanguageClient;

const ENVY_LANGUAGE_SERVER_SOCKET_PATH = "/tmp/envy_language_server.socket";

export function activate() {
  const serverOptions: ServerOptions = () => {
    const socket = connect({ path: ENVY_LANGUAGE_SERVER_SOCKET_PATH });
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
    "envyLanguageServer",
    "Envy Language Server",
    serverOptions,
    clientOptions,
  );

  client.start();
}

export function deactivate(): Thenable<void> | undefined {
  if (!client) {
    return undefined;
  }

  return client.stop();
}
