export class MyMap {
    private map = new Map<string, any>();

    set(key: [number, number], value: any): this {
        this.map.set(JSON.stringify(key), value);
        return this
    }

    get(key: [number, number]): any | undefined {
        return this.map.get(JSON.stringify(key));
    }

    clear() {
        this.map.clear();
    }

    delete(key: [number, number]): boolean {
        return this.map.delete(JSON.stringify(key));
    }

    has(key: [number, number]): boolean {
        return this.map.has(JSON.stringify(key));
    }

    get size() {
        return this.map.size;
    }

    // Something is not quite right
    forEach(callbackfn: (value: any, key: [number, number], mapArg: Map<[number, number], any>) => void, thisArg?: any): void {
        this.map.forEach((value, key) => {
            callbackfn.call(thisArg, value, JSON.parse(key), thisArg);
        });
    }
}