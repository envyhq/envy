import { Socket } from "net";

export interface EnvyClient {
  socket: Socket;
  HOME: () => Promise<string>;
}

export function createEnvySocketClient() {
  return new Promise<EnvyClient>((resolve, reject) => {
    const socket = new Socket();

    socket.connect("/tmp/env.provider.nv.sock");

    const getValue = (key: string) => {
      return new Promise<string>((resolve, reject) => {
        socket.write(key, (error) => {
          if (error) {
            reject(error);
          }
        });
        socket.on("data", (data) => {
          resolve(data.toString());
        });
      });
    };

    socket.on("connect", () => {
      resolve({ socket, HOME: () => getValue("HOME") });
    });

    socket.on("error", (error) => {
      reject(error);
    });
  });
}

const test = async () => {
  const client = await createEnvySocketClient();

  console.log({ client });

  client.socket.on("data", (data) => {
    console.log("data", data.toString());
  });

  console.log("waiting to write...");
  console.log("writing");
  const result = await client.HOME();
  console.log("wrote", { result });
};

test();
