export type Frame = {
    type: 'req' | 'res' | 'event';
    id?: string;
    method?: string;
    event?: string;
    params?: any;
    payload?: any;
    ok?: boolean;
    error?: string | null;
};

class GatewayClient {
    private ws: WebSocket | null = null;
    private handlers: Map<string, (payload: any) => void> = new Map();
    private pendingRequests: Map<string, (res: Frame) => void> = new Map();

    constructor(private url: string = 'ws://localhost:18789/ws') { }

    connect() {
        return new Promise((resolve, reject) => {
            this.ws = new WebSocket(this.url);

            this.ws.onopen = () => {
                console.log('🌑 Sovereign Gateway Connected');
                this.authenticate().then(resolve).catch(reject);
            };

            this.ws.onmessage = (event) => {
                const frame: Frame = JSON.parse(event.data);
                this.handleFrame(frame);
            };

            this.ws.onclose = () => {
                console.warn('🌑 Sovereign Gateway Disconnected. Retrying...');
                setTimeout(() => this.connect(), 5000);
            };

            this.ws.onerror = (err) => {
                console.error('Gateway Error:', err);
                reject(err);
            };
        });
    }

    private async authenticate() {
        return this.request('connect', {
            minProtocol: 3,
            maxProtocol: 3,
            client: { id: 'sarah-body-ui', version: '1.0.0', platform: 'web' },
            auth: { token: 'sovereign-ui-token' }
        });
    }

    request(method: string, params: any): Promise<Frame> {
        const id = Math.random().toString(36).substring(7);
        const frame: Frame = { type: 'req', id, method, params };

        return new Promise((resolve) => {
            this.pendingRequests.set(id, resolve);
            this.ws?.send(JSON.stringify(frame));
        });
    }

    onEvent(event: string, handler: (payload: any) => void) {
        this.handlers.set(event, handler);
    }

    private handleFrame(frame: Frame) {
        if (frame.type === 'res' && frame.id && this.pendingRequests.has(frame.id)) {
            const resolve = this.pendingRequests.get(frame.id);
            resolve?.(frame);
            this.pendingRequests.delete(frame.id);
        } else if (frame.type === 'event' && frame.event && this.handlers.has(frame.event)) {
            const handler = this.handlers.get(frame.event);
            handler?.(frame.payload);
        }
    }
}

export const gateway = new GatewayClient();
