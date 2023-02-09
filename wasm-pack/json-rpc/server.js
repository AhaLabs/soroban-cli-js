import { Server as JsServer } from './stellar-sdk.js';

export class Server {
    constructor(url) {
        this.server = new JsServer(url);
    }

    async friendbot(address) {
        return this.server.friendbot(address).call();
    }

    async sendTransaction(transaction) {
        return this.server.submitTransaction(transaction);
    }

}