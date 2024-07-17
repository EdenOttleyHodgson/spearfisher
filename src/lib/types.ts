interface FileManagerData {
  current_location: FileData,
  current_files: FileData[]
  searched_term: string
}

interface FileData {
  name: string,
  full_path: string,
  file_type: FileType,
  metadata: Metadata
}

interface Metadata {
  length: number,
  readonly: boolean,
  modified: number,
  accessed: number,
  created: number
}

type FileType = "File" | "Directory" | "Symlink"
