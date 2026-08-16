export interface FileEntry {
  name: string
  path: string
  is_dir: boolean
  size: number
  format: string
  modified: string
  created: string
}

export interface DirEntry {
  name: string
  path: string
  is_dir: boolean
  children?: DirEntry[]
}

export interface FileDetail {
  name: string
  path: string
  format: string
  size: number
  modified: string
  created: string
  is_dir: boolean
}

export interface ExifInfo {
  has_exif: boolean
  camera_make?: string
  camera_model?: string
  date_taken?: string
  gps_latitude?: number
  gps_longitude?: number
  gps_altitude?: number
  width?: number
  height?: number
  orientation?: number
  iso?: number
  focal_length?: string
  aperture?: string
  shutter_speed?: string
}

export interface GpsLocation {
  latitude: number
  longitude: number
  province?: string
  city?: string
  district?: string
  place?: string
}

export interface FileDetailInfo {
  basic: FileDetail
  exif?: ExifInfo
  gps?: GpsLocation
  ai_tags: string[]
}

export interface LogEntry {
  id: string
  timestamp: string
  operation_type: string
  source_path: string
  target_path?: string
  status: string
  detail?: string
}

export interface LayoutState {
  showLeftBar: boolean
  showRightBar: boolean
  leftBarWidth: number
  rightBarWidth: number
}

export interface WorkDirectory {
  path: string
  name: string
}

export interface DirectoryStats {
  total_files: number
  total_dirs: number
  total_size: number
  file_types: Record<string, number>
  oldest_file?: string
  newest_file?: string
}

export interface OrganizeRule {
  tags: string[]
  time_source: string
}

export interface OrganizeResult {
  source_path: string
  target_path: string
  success: boolean
  error?: string
  metadata?: OrganizeMetadata
  process_time_ms?: number  // 处理耗时（毫秒）
}

export interface OrganizeMetadata {
  modified?: string
  created?: string
  taken?: string
  gps_latitude?: number
  gps_longitude?: number
  gps_province?: string
  gps_city?: string
  gps_district?: string
  gps_place?: string
  camera_make?: string
  camera_model?: string
  size?: number
  format?: string
}

export interface RenameRule {
  template_parts: TemplatePart[]
  time_source: string
  start_index: number
}

export interface TemplatePart {
  part_type: string
  value: string
}

export interface RenameResult {
  source_path: string
  new_name: string
  target_path: string
  success: boolean
  error?: string
}

export interface DuplicateGroup {
  md5: string
  size: number
  files: DuplicateFile[]
}

export interface DuplicateFile {
  path: string
  name: string
  modified: string
  is_original: boolean
}

export interface DuplicateScanResult {
  total_files: number
  duplicate_groups: DuplicateGroup[]
  total_duplicates: number
  wasted_space: number
}

export interface CleanupResult {
  cleanup_type: string
  files: CleanupFile[]
  total_size: number
}

export interface CleanupFile {
  path: string
  name: string
  size: number
  success: boolean
  error?: string
}

export interface BatchOperationResult {
  total: number
  success_count: number
  fail_count: number
  results: OperationResult[]
}

export interface OperationResult {
  success: boolean
  message?: string
  error?: string
}

export type FunctionPanel =
  | 'home'
  | 'file-organize'
  | 'batch-rename'
  | 'duplicate-clean'
  | 'cleanup'
  | 'fix-date'
  | 'exif-clean'
  | 'write-gps'
  | 'ai-classify'
  | 'log-view'
  | 'log-detail'
  | 'settings'
  | 'privacy-policy'
  | 'license-agreement'

export interface PageState {
  [key: string]: any
}

export interface OrganizeProgress {
  total: number
  processed: number
  success_count: number
  fail_count: number
  current_file?: string
  percentage: number
}

export interface TaskProgress {
  total: number
  processed: number
  current_item?: string
  percentage: number
}

export interface CancelResult {
  cancelled: boolean
  message: string
}
