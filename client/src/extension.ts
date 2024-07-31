/* --------------------------------------------------------------------------------------------
 * Copyright (c) Microsoft Corporation. All rights reserved.
 * Licensed under the MIT License. See License.txt in the project root for license information.
 * ------------------------------------------------------------------------------------------ */

import {
   workspace,
   ExtensionContext,
   window,
   StatusBarAlignment,
   StatusBarItem
} from "vscode";

import * as path from "path";

import {
   Executable,
   LanguageClient,
   LanguageClientOptions,
   ServerOptions,
   TransportKind,
} from "vscode-languageclient/node";

let client: LanguageClient;
let statusBarItem: StatusBarItem;

export async function activate(context: ExtensionContext) {

   const traceOutputChannel = window.createOutputChannel(
      "Stack Language Server trace1"
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

   // Create the language client and start the client.
   client = new LanguageClient(
      "stack-language-server",
      "stack language server",
      serverOptions,
      clientOptions
   );
   
   const myCommandId = 'custom/statusBar';
   // Create a status bar item
   statusBarItem = window.createStatusBarItem(StatusBarAlignment.Right, 100);
   context.subscriptions.push(statusBarItem);


   client.onNotification(myCommandId, (params) => {
      if (params.text != "") {
         statusBarItem.text = params.text;
         statusBarItem.show();
      } else {
         statusBarItem.hide();
      }
   });

   client.start();
}

export function deactivate(): Thenable<void> | undefined {
   if (!client) {
      return undefined;
   }
   return client.stop();
}
