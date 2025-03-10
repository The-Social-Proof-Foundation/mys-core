// Copyright (c) The Diem Core Contributors
// Copyright (c) The Move Contributors
// SPDX-License-Identifier: Apache-2.0

import * as os from 'os';
import * as vscode from 'vscode';
import * as path from 'path';

export const MOVE_CONF_NAME = 'move';
export const LINT_OPT = 'lint';
export const TYPE_HINTS_OPT = 'inlay-hints.type';
export const PARAM_HINTS_OPT = 'inlay-hints.param';
export const MYS_PATH_OPT = 'mys.path';
export const SERVER_PATH_OPT = 'server.path';

/**
 * User-defined configuration values, such as those specified in VS Code settings.
 *
 * This provides a more strongly typed interface to the configuration values specified in this
 * extension's `package.json`, under the key `"contributes.configuration.properties"`.
 */
export class Configuration {
    private readonly configuration: vscode.WorkspaceConfiguration;

    /** Default directory for the location of the language server binary */
    readonly defaultServerDir: vscode.Uri;

    /** Name of the language server binary */
    readonly serverName: string;

    /** Default path to the language server binary */
    readonly defaultServerPath: vscode.Uri;

    constructor() {
        this.configuration = vscode.workspace.getConfiguration(MOVE_CONF_NAME);
        this.defaultServerDir = vscode.Uri.joinPath(vscode.Uri.file(os.homedir()), '.mys', 'bin');
        if (process.platform === 'win32') {
            this.serverName = 'move-analyzer.exe';
        } else {
            this.serverName = 'move-analyzer';
        }
        this.defaultServerPath = vscode.Uri.joinPath(this.defaultServerDir, this.serverName);
    }

    /** A string representation of the configured values, for logging purposes. */
    toString(): string {
        return JSON.stringify(this.configuration);
    }

    /** The path to the move-analyzer executable. */
    get serverPath(): string {
        const serverPath = this.configuration.get<string | null>(SERVER_PATH_OPT) ?? this.defaultServerPath.fsPath;
        if (serverPath.startsWith('~/')) {
            return os.homedir() + serverPath.slice('~'.length);
        }
        return path.resolve(serverPath);
    }

    /** The path to the Mys binary. */
    get mysPath(): string {
        const mysBin = process.platform === 'win32' ? 'mys.exe' : 'mys';
        const mysPath = this.configuration.get<string | null>(MYS_PATH_OPT) ?? mysBin;

        if (mysPath === mysBin) {
            return mysPath;
        }
        if (mysPath.startsWith('~/')) {
            return os.homedir() + mysPath.slice('~'.length);
        }
        return path.resolve(mysPath);
    }

    get lint(): string {
        return this.configuration.get(LINT_OPT) ?? 'default';
    }

    get inlayHintsForType(): boolean {
        return this.configuration.get(TYPE_HINTS_OPT) ?? false;
    }

    get inlayHintsForParam(): boolean {
        return this.configuration.get(PARAM_HINTS_OPT) ?? false;
    }


} // Configuration
