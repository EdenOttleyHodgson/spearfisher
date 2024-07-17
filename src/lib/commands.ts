import { invoke } from "@tauri-apps/api/tauri";

export async function get_file_manager_data(): Promise<FileManagerData> {
  let file_manager_data_string: string = await invoke('get_file_manager_data')
  return JSON.parse(file_manager_data_string);
}
