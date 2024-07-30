/* --------------------------------------------------------------------------------------------
 * Copyright (c) Microsoft Corporation. All rights reserved.
 * Licensed under the MIT License. See License.txt in the project root for license information.
 * ------------------------------------------------------------------------------------------ */

import {
  languages,
  workspace,
  EventEmitter,
  ExtensionContext,
  window,
  InlayHintsProvider,
  TextDocument,
  CancellationToken,
  Range,
  InlayHint,
  TextDocumentChangeEvent,
  ProviderResult,
  commands,
  WorkspaceEdit,
  TextEdit,
  Selection,
  Uri,
} from "vscode";

import * as path from "path";

import {
  Disposable,
  Executable,
  LanguageClient,
  LanguageClientOptions,
  ServerOptions,
  TransportKind,
} from "vscode-languageclient/node";

let client: LanguageClient;

export async function activate(context: ExtensionContext) {
  window.showInformationMessage("activate");

  const traceOutputChannel = window.createOutputChannel(
    "Stack Language Server trace"
  );
  const command =
    process.env.SERVER_PATH ||
    path.join(
      context.extensionPath,
      "server",
      "target",
      "release",
      "stack-language-server.exe"
    );

  window.showInformationMessage(command);

  const run: Executable = {
    command,
    transport: TransportKind.stdio,
    options: {
      env: {
        ...process.env,
        // eslint-disable-next-line @typescript-eslint/naming-convention
        RUST_LOG: "debug",
      },
    },
  };
  const serverOptions: ServerOptions = {
    run,
    debug: run,
  };

  console.log(serverOptions);

  // If the extension is launched in debug mode then the debug server options are used
  // Otherwise the run options are used
  // Options to control the language client
  let clientOptions: LanguageClientOptions = {
    // Register the server for plain text documents
    documentSelector: [{ scheme: "file", language: "stack" }],
    synchronize: {
      // Notify the server about file changes to '.clientrc files contained in the workspace
      fileEvents: [
        workspace.createFileSystemWatcher("**/.prg"),
        workspace.createFileSystemWatcher("**/.hdl"),
      ],
    },
    traceOutputChannel,
  };

  window.showInformationMessage("new client");

  // Create the language client and start the client.
  client = new LanguageClient(
    "stack-language-server",
    "stack language server",
    serverOptions,
    clientOptions
  );

  window.showInformationMessage("starting client");

  client.start();

  window.showInformationMessage("client started");
}

export function deactivate(): Thenable<void> | undefined {
  if (!client) {
    return undefined;
  }
  return client.stop();
}
