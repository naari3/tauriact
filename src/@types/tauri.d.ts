declare module "tauri/api/window" {
  export function open(url: string): void;
  export function setTitle(title: string): void;
}
