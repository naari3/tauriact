declare module "tauri/api/dialog" {
  export function open(options = {}): Promise<string>;
  export function save(options = {}): Promise<string>;
}
