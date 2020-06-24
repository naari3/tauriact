type APICall = "getTweetCount";
type API = { [key in APICall]?: any };

const tauriAPI = (args: any) => {
  const generateID = (): string => {
    return Math.random().toString(36).substr(2, 9);
  };

  const transformCallback = (callback: (payload: any) => void) => {
    const id = generateID();
    const win = window as any;
    win[id] = (result: any) => {
      delete win[id];
      return callback && callback(result);
    };
    return id;
  };

  return new Promise<string>((resolve, reject) => {
    const win = window as any;
    win.invoke(
      JSON.stringify({
        callback: transformCallback(resolve),
        error: transformCallback(reject),
        ...args,
      })
    );
  });
};

const Middleware = {
  get: (_target: Record<string, unknown>, cmd: APICall) => {
    return async (payload: Record<string, unknown>): Promise<string> => {
      return tauriAPI({ cmd, ...payload });
    };
  },
};

const API = new Proxy<API>({}, Middleware);

export default API;
