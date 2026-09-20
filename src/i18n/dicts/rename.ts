import type { Dict } from '../types'

const dict: Dict = {
  // 页面
  'rename.title': { 'zh-CN': '批量重命名', 'zh-TW': '批次重新命名', en: 'Batch Rename' },
  'rename.desc': { 'zh-CN': '用灵活模板批量修改文件名，让文件命名更规范', 'zh-TW': '用靈活模板批次修改檔案名，讓檔案命名更規範', en: 'Batch-edit file names with flexible templates for consistent naming' },
  'rename.timeSourceLabel': { 'zh-CN': '时间数据来源', 'zh-TW': '時間資料來源', en: 'Time data source' },
  'rename.timeSourcePlaceholder': { 'zh-CN': '选择时间来源', 'zh-TW': '選擇時間來源', en: 'Select a time source' },
  'rename.timeSourceModified': { 'zh-CN': '修改时间', 'zh-TW': '修改時間', en: 'Modified time' },
  'rename.timeSourceCreated': { 'zh-CN': '创建时间', 'zh-TW': '建立時間', en: 'Created time' },
  'rename.timeSourceTaken': { 'zh-CN': '拍摄日期', 'zh-TW': '拍攝日期', en: 'Date taken' },
  'rename.timeSourceHint': { 'zh-CN': '用于日期、年份、月份、时间等标签', 'zh-TW': '用於日期、年份、月份、時間等標籤', en: 'Used by date, year, month and time tags' },
  'rename.startIndexLabel': { 'zh-CN': '序号起始值', 'zh-TW': '序號起始值', en: 'Start index' },
  'rename.tagsLabel': { 'zh-CN': '命名标签（点击添加，可重复选择）', 'zh-TW': '命名標籤（點擊添加，可重複選擇）', en: 'Name tags (click to add, repeatable)' },
  'rename.currentTemplateLabel': { 'zh-CN': '当前模板（点击移除）', 'zh-TW': '當前模板（點擊移除）', en: 'Current template (click to remove)' },

  // 进度与结果
  'rename.progressTitle': { 'zh-CN': '正在重命名文件', 'zh-TW': '正在重新命名檔案', en: 'Renaming files' },
  'rename.progressTotal': { 'zh-CN': '总数: {n}', 'zh-TW': '總數: {n}', en: 'Total: {n}' },
  'rename.progressProcessed': { 'zh-CN': '已处理: {n}', 'zh-TW': '已處理: {n}', en: 'Processed: {n}' },
  'rename.progressCurrentLabel': { 'zh-CN': '当前:', 'zh-TW': '當前:', en: 'Current:' },
  'rename.startButton': { 'zh-CN': '开始重命名', 'zh-TW': '開始重新命名', en: 'Start rename' },
  'rename.stopButton': { 'zh-CN': '终止重命名', 'zh-TW': '終止重新命名', en: 'Stop rename' },
  'rename.resultTitle': { 'zh-CN': '重命名结果', 'zh-TW': '重新命名結果', en: 'Rename results' },
  'rename.resultStats': { 'zh-CN': '成功: {success} / 失败: {fail}', 'zh-TW': '成功: {success} / 失敗: {fail}', en: 'Success: {success} / Failed: {fail}' },
  'rename.failCountLabel': { 'zh-CN': '失败 {n} 个', 'zh-TW': '失敗 {n} 個', en: '{n} failed' },
  'rename.successCountLabel': { 'zh-CN': '成功 {n} 个', 'zh-TW': '成功 {n} 個', en: '{n} succeeded' },

  // 预览
  'rename.previewEmpty': { 'zh-CN': '点击标签构建模板', 'zh-TW': '點擊標籤構建模板', en: 'Click tags to build a template' },
  'rename.example.type': { 'zh-CN': '图片', 'zh-TW': '圖片', en: 'Photo' },
  'rename.example.province': { 'zh-CN': '江西省', 'zh-TW': '江西省', en: 'Jiangxi' },
  'rename.example.city': { 'zh-CN': '南昌市', 'zh-TW': '南昌市', en: 'Nanchang' },
  'rename.example.district': { 'zh-CN': '红谷滩区', 'zh-TW': '紅谷灘區', en: 'Honggutan' },
  'rename.example.place': { 'zh-CN': '江西科技师范大学', 'zh-TW': '江西科技師範大學', en: 'JiangxiNormalUniv' },

  // 通知
  'rename.notif.infoTitle': { 'zh-CN': '提示', 'zh-TW': '提示', en: 'Notice' },
  'rename.notif.warningTitle': { 'zh-CN': '警告', 'zh-TW': '警告', en: 'Warning' },
  'rename.notif.errorTitle': { 'zh-CN': '错误', 'zh-TW': '錯誤', en: 'Error' },
  'rename.notif.successTitle': { 'zh-CN': '成功', 'zh-TW': '成功', en: 'Success' },
  'rename.stopInProgress': { 'zh-CN': '正在终止重命名操作...', 'zh-TW': '正在終止重新命名操作...', en: 'Stopping the rename operation...' },
  'rename.stopFailed': { 'zh-CN': '终止失败: {msg}', 'zh-TW': '終止失敗: {msg}', en: 'Failed to stop: {msg}' },
  'rename.warnNoTemplate': { 'zh-CN': '请先构建命名模板', 'zh-TW': '請先構建命名模板', en: 'Build a naming template first' },
  'rename.warnNoWorkDir': { 'zh-CN': '请先选择待处理目录', 'zh-TW': '請先選擇待處理目錄', en: 'Select a directory to process first' },
  'rename.warnNoOutputDir': { 'zh-CN': '请先选择输出目录', 'zh-TW': '請先選擇輸出目錄', en: 'Select an output directory first' },
  'rename.allSuccess': { 'zh-CN': '重命名完成，共处理 {n} 个文件，已复制到输出目录', 'zh-TW': '重新命名完成，共處理 {n} 個檔案，已複製到輸出目錄', en: 'Rename complete: {n} files processed and copied to the output directory' },
  'rename.partialSuccess': { 'zh-CN': '重命名完成，成功 {success} 个，失败 {fail} 个', 'zh-TW': '重新命名完成，成功 {success} 個，失敗 {fail} 個', en: 'Rename complete: {success} succeeded, {fail} failed' },
  'rename.noFiles': { 'zh-CN': '没有找到需要重命名的文件', 'zh-TW': '沒有找到需要重新命名的檔案', en: 'No files found to rename' },
  'rename.cancelled': { 'zh-CN': '重命名操作已取消', 'zh-TW': '重新命名操作已取消', en: 'Rename operation cancelled' },
  'rename.failed': { 'zh-CN': '重命名失败: {msg}', 'zh-TW': '重新命名失敗: {msg}', en: 'Rename failed: {msg}' },

  // 标签
  'rename.tag.date': { 'zh-CN': '日期', 'zh-TW': '日期', en: 'Date' },
  'rename.tag.time': { 'zh-CN': '时间', 'zh-TW': '時間', en: 'Time' },
  'rename.tag.year': { 'zh-CN': '年份', 'zh-TW': '年份', en: 'Year' },
  'rename.tag.month': { 'zh-CN': '月份', 'zh-TW': '月份', en: 'Month' },
  'rename.tag.day': { 'zh-CN': '日', 'zh-TW': '日', en: 'Day' },
  'rename.tag.type': { 'zh-CN': '文件类型', 'zh-TW': '檔案類型', en: 'File type' },
  'rename.tag.name': { 'zh-CN': '原文件名', 'zh-TW': '原始檔名', en: 'Original name' },
  'rename.tag.ext': { 'zh-CN': '扩展名', 'zh-TW': '副檔名', en: 'Extension' },
  'rename.tag.index': { 'zh-CN': '序号', 'zh-TW': '序號', en: 'Index' },
  'rename.tag.province': { 'zh-CN': '省份', 'zh-TW': '省份', en: 'Province' },
  'rename.tag.city': { 'zh-CN': '城市', 'zh-TW': '城市', en: 'City' },
  'rename.tag.district': { 'zh-CN': '区县', 'zh-TW': '區縣', en: 'District' },
  'rename.tag.place': { 'zh-CN': '地点', 'zh-TW': '地點', en: 'Place' },
  'rename.tag.make': { 'zh-CN': '相机品牌', 'zh-TW': '相機品牌', en: 'Camera make' },
  'rename.tag.model': { 'zh-CN': '相机型号', 'zh-TW': '相機型號', en: 'Camera model' },
  'rename.tag.exact_size': { 'zh-CN': '文件大小', 'zh-TW': '檔案大小', en: 'File size' },
  'rename.tag.space': { 'zh-CN': '空格', 'zh-TW': '空格', en: 'Space' },
}

export default dict
