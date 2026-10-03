import type { Dict } from '../types'

const dict: Dict = {
  // ===== AI 智能分类 =====
  'ai.title': {
    'zh-CN': 'AI 智能分类',
    'zh-TW': 'AI 智慧分類',
    'en': 'AI Smart Classification',
  },
  'ai.desc': {
    'zh-CN': '使用本地 AI 模型自动识别照片内容，按类别整理到子文件夹',
    'zh-TW': '使用本地 AI 模型自動辨識照片內容，按類別整理到子資料夾',
    'en': 'Use local AI model to auto-recognize photo content and sort into subfolders',
  },

  // 当前模型显示
  'ai.model.current': {
    'zh-CN': '当前模型',
    'zh-TW': '目前模型',
    'en': 'Current Model',
  },
  'ai.model.hint': {
    'zh-CN': '可在设置中更换识别模型',
    'zh-TW': '可在設定中更換識別模型',
    en: 'Change model in Settings',
  },
  'ai.confidenceThreshold': {
    'zh-CN': '置信度阈值',
    'zh-TW': '信心度閾值',
    en: 'Confidence Threshold',
  },
  'ai.threshold.loose': {
    'zh-CN': '宽松',
    'zh-TW': '寬鬆',
    en: 'Loose',
  },
  'ai.threshold.standard': {
    'zh-CN': '标准',
    'zh-TW': '標準',
    en: 'Standard',
  },
  'ai.threshold.strict': {
    'zh-CN': '严格',
    'zh-TW': '嚴格',
    en: 'Strict',
  },
  'ai.threshold.ultra': {
    'zh-CN': '极严格',
    'zh-TW': '極嚴格',
    en: 'Very Strict',
  },

  // 进度
  'ai.progress.title': {
    'zh-CN': '正在分类图片',
    'zh-TW': '正在分類圖片',
    'en': 'Classifying Images',
  },
  'ai.progress.scanned': {
    'zh-CN': '已扫描 {n} 张',
    'zh-TW': '已掃描 {n} 張',
    'en': '{n} images scanned',
  },

  // 操作按钮
  'ai.action.start': {
    'zh-CN': '开始分类',
    'zh-TW': '開始分類',
    'en': 'Start Classification',
  },
  'ai.action.stop': {
    'zh-CN': '终止分类',
    'zh-TW': '終止分類',
    'en': 'Stop Classification',
  },
  // 结果
  'ai.result.title': {
    'zh-CN': '分类结果',
    'zh-TW': '分類結果',
    'en': 'Classification Results',
  },
  'ai.result.total': {
    'zh-CN': '共 {n} 张图片',
    'zh-TW': '共 {n} 張圖片',
    'en': '{n} images total',
  },
  'ai.result.categoryCount': {
    'zh-CN': '{cat}: {n} 张',
    'zh-TW': '{cat}: {n} 張',
    'en': '{cat}: {n} images',
  },
  'ai.result.otherCategories': {
    'zh-CN': '其他',
    'zh-TW': '其他',
    'en': 'Other',
  },
  'ai.result.confidence': {
    'zh-CN': '置信度: {p}%',
    'zh-TW': '信心度: {p}%',
    'en': 'Confidence: {p}%',
  },
  'ai.result.organized': {
    'zh-CN': '已归档 {n} 个文件到输出目录',
    'zh-TW': '已歸檔 {n} 個檔案到輸出目錄',
    'en': '{n} files organized to output folder',
  },

  // 通知
  'ai.doneAll': {
    'zh-CN': '分类完成，共处理 {n} 张图片',
    'zh-TW': '分類完成，共處理 {n} 張圖片',
    'en': 'Classification complete, {n} images processed',
  },
  'ai.donePartial': {
    'zh-CN': '分类完成，成功 {s} 张，失败 {f} 张',
    'zh-TW': '分類完成，成功 {s} 張，失敗 {f} 張',
    'en': 'Classification complete, {s} succeeded, {f} failed',
  },
  'ai.cancelled': {
    'zh-CN': '分类操作已取消',
    'zh-TW': '分類操作已取消',
    'en': 'Classification cancelled',
  },
  'ai.failed': {
    'zh-CN': '分类失败: {e}',
    'zh-TW': '分類失敗: {e}',
    'en': 'Classification failed: {e}',
  },
  'ai.noModel': {
    'zh-CN': '请先在设置中添加模型',
    'zh-TW': '請先在設定中加入模型',
    'en': 'Please add a model in Settings first',
  },
  'ai.noImages': {
    'zh-CN': '未找到可分类的图片',
    'zh-TW': '未找到可分類的圖片',
    'en': 'No classifiable images found',
  },

  // ===== 人脸聚类 =====
  'ai.mode': {
    'zh-CN': '功能模式',
    'zh-TW': '功能模式',
    en: 'Mode',
  },
  'ai.mode.content': {
    'zh-CN': '内容分类',
    'zh-TW': '內容分類',
    en: 'Content',
  },
  'ai.mode.face': {
    'zh-CN': '人物分组',
    'zh-TW': '人物分組',
    en: 'People',
  },
  'ai.mode.hybrid': {
    'zh-CN': '混合模式',
    'zh-TW': '混合模式',
    en: 'Hybrid',
  },
  'ai.mode.hybridDesc': {
    'zh-CN': '先识别有人脸的照片按人物分组，其余按内容分类',
    'zh-TW': '先辨識有人臉的照片按人物分組，其餘按內容分類',
    en: 'Face-group photos with people, classify remaining by content',
  },
  'ai.face.modelHint': {
    'zh-CN': '使用 YuNet 检测 + SFace 人脸识别',
    'zh-TW': '使用 YuNet 檢測 + SFace 人臉識別',
    en: 'YuNet detection + SFace recognition',
  },
  'ai.face.start': {
    'zh-CN': '开始分析',
    'zh-TW': '開始分析',
    en: 'Start Analysis',
  },
  'ai.face.stop': {
    'zh-CN': '终止分析',
    'zh-TW': '終止分析',
    en: 'Stop Analysis',
  },
  'ai.face.progressTitle': {
    'zh-CN': '正在分析人脸',
    'zh-TW': '正在分析人臉',
    en: 'Analyzing Faces',
  },
  'ai.face.resultTitle': {
    'zh-CN': '人物分组结果',
    'zh-TW': '人物分組結果',
    en: 'People Grouping Results',
  },
  'ai.face.photoCount': {
    'zh-CN': '{n} 张照片',
    'zh-TW': '{n} 張照片',
    en: '{n} photos',
  },
  'ai.face.rename': {
    'zh-CN': '重命名',
    'zh-TW': '重新命名',
    en: 'Rename',
  },
  'ai.face.renamePrompt': {
    'zh-CN': '输入人物名称',
    'zh-TW': '輸入人物名稱',
    en: 'Enter person name',
  },
  'ai.face.merge': {
    'zh-CN': '合并',
    'zh-TW': '合併',
    en: 'Merge',
  },
  'ai.face.mergeTitle': {
    'zh-CN': '合并到其他人物',
    'zh-TW': '合併到其他人物',
    en: 'Merge into Another Person',
  },
  'ai.face.mergeTarget': {
    'zh-CN': '合并到',
    'zh-TW': '合併到',
    en: 'Merge into',
  },
  'ai.face.archive': {
    'zh-CN': '按人物归档',
    'zh-TW': '按人物歸檔',
    en: 'Archive by Person',
  },
  'ai.face.archiveNeedOutDir': {
    'zh-CN': '请先在首页设置保存位置',
    'zh-TW': '請先在首頁設定儲存位置',
    en: 'Set the output directory on the home page first',
  },
  'ai.face.archived': {
    'zh-CN': '已按人物归档 {n} 个文件',
    'zh-TW': '已按人物歸檔 {n} 個檔案',
    en: 'Archived {n} files by person',
  },
  'ai.face.done': {
    'zh-CN': '人脸分析完成，共 {n} 个人物',
    'zh-TW': '人臉分析完成，共 {n} 個人物',
    en: 'Face analysis complete, {n} people found',
  },
  'ai.face.noFaces': {
    'zh-CN': '未检测到人脸',
    'zh-TW': '未檢測到人臉',
    en: 'No faces detected',
  },
  'ai.face.failed': {
    'zh-CN': '人脸分析失败: {e}',
    'zh-TW': '人臉分析失敗: {e}',
    en: 'Face analysis failed: {e}',
  },
  'ai.model.fast': { 'zh-CN': '快速', 'zh-TW': '快速', en: 'Fast' },
  'ai.model.accurate': { 'zh-CN': '精准', 'zh-TW': '精準', en: 'Accurate' },
  'ai.result.hybrid': { 'zh-CN': '{faceCount} 张人脸, {clsCount} 张内容分类', 'zh-TW': '{faceCount} 張人臉, {clsCount} 張內容分類', en: '{faceCount} faces, {clsCount} content-classified' },
  // 日志操作名
  'ai.log.contentClassify': { 'zh-CN': 'AI内容分类', 'zh-TW': 'AI內容分類', en: 'AI Content Classification' },
  'ai.log.faceCluster': { 'zh-CN': '人脸聚类', 'zh-TW': '人臉聚類', en: 'Face Clustering' },
  'ai.log.hybridClassify': { 'zh-CN': '混合模式分类', 'zh-TW': '混合模式分類', en: 'Hybrid Classification' },
  'ai.log.personRename': { 'zh-CN': '人物改名: {name}', 'zh-TW': '人物改名: {name}', en: 'Person renamed: {name}' },
  'ai.log.personMerge': { 'zh-CN': '人物合并: {source} → {target}', 'zh-TW': '人物合併: {source} → {target}', en: 'Person merged: {source} → {target}' },
  'ai.log.personArchive': { 'zh-CN': '按人物归档: 成功 {success}/{total} 个文件, 输出目录: {dir}', 'zh-TW': '按人物歸檔: 成功 {success}/{total} 個檔案, 輸出目錄: {dir}', en: 'Archive by person: {success}/{total} files succeeded, output: {dir}' },
}

export default dict
