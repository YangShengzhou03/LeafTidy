import type { Dict } from '../types'

// 隐私政策页面（PrivacyPolicyPage.vue）
const dict: Dict = {
  'legal.privacy.title': {
    'zh-CN': '隐私政策',
    'zh-TW': '隱私權政策',
    en: 'Privacy Policy',
  },
  'legal.privacy.meta': {
    'zh-CN': '更新日期：2026年9月13日 · 开发者：杨圣洲 · 适用版本：v1.0.0 及以上',
    'zh-TW': '更新日期：2026年9月13日 · 開發者：楊聖洲 · 適用版本：v1.0.0 及以上',
    en: 'Last Updated: September 13, 2026 · Developer: 杨圣洲 · Applicable Version: v1.0.0 and above',
  },

  'legal.privacy.s1': {
    'zh-CN': '一、引言',
    'zh-TW': '一、引言',
    en: '1. Introduction',
  },
  'legal.privacy.s1p1': {
    'zh-CN': '轻羽归档（LeafTidy，以下简称"本应用"）是由开发者杨圣洲（以下简称"开发者"）开发和发布的本地文件与照片整理工具。用户个人信息与数据安全是本应用设计的首要原则。本《隐私政策》（以下简称"本政策"）旨在向您清晰、完整地说明：本应用处理哪些信息、以何种方式处理、信息的存储与删除机制，以及您对自身数据享有的完整权利。',
    'zh-TW': '輕羽歸檔（LeafTidy，以下簡稱「本應用」）是由開發者楊聖洲（以下簡稱「開發者」）開發和發布的本機檔案與照片整理工具。使用者個人資訊與資料安全是本應用設計的首要原則。本《隱私權政策》（以下簡稱「本政策」）旨在向您清晰、完整地說明：本應用處理哪些資訊、以何種方式處理、資訊的儲存與刪除機制，以及您對自身資料享有的完整權利。',
    en: 'LeafTidy ("the Application", Chinese name 轻羽归档) is a local file and photo organizing tool developed and released by the developer 杨圣洲 ("the Developer"). The security of personal information and data of users is the foremost principle in the design of the Application. This Privacy Policy ("this Policy") is intended to clearly and comprehensively explain to you: what information the Application processes, in what manner it is processed, the mechanisms for storage and deletion of such information, and the complete rights you hold over your own data.',
  },
  'legal.privacy.s1p2': {
    'zh-CN': '请您在使用本应用前，仔细阅读并充分理解本政策的全部内容，特别是以加粗形式提示的条款。一旦您下载、安装或开始使用本应用，即视为您已理解并同意本政策的全部内容。若您不同意本政策的任何条款，请立即停止安装或使用本应用。',
    'zh-TW': '請您在使用本應用前，仔細閱讀並充分理解本政策的全部內容，特別是以粗體形式提示的條款。一旦您下載、安裝或開始使用本應用，即視為您已理解並同意本政策的全部內容。若您不同意本政策的任何條款，請立即停止安裝或使用本應用。',
    en: 'Before using the Application, please read the entire contents of this Policy carefully and ensure that you fully understand them, in particular the provisions highlighted in bold. Once you download, install, or begin to use the Application, you shall be deemed to have understood and agreed to the entire contents of this Policy. If you do not agree to any provision of this Policy, please immediately cease installing or using the Application.',
  },

  'legal.privacy.s2': {
    'zh-CN': '二、适用范围',
    'zh-TW': '二、適用範圍',
    en: '2. Scope of Application',
  },
  'legal.privacy.s2p1': {
    'zh-CN': '本政策适用于本应用的全部功能模块，包括但不限于：',
    'zh-TW': '本政策適用於本應用的全部功能模組，包括但不限於：',
    en: 'This Policy applies to all functional modules of the Application, including but not limited to:',
  },
  'legal.privacy.s2l1': {
    'zh-CN': '文件整理归档：按时间、拍摄地点、设备等维度整理照片与文件',
    'zh-TW': '檔案整理歸檔：按時間、拍攝地點、裝置等維度整理照片與檔案',
    en: 'File organization and archiving: organizing photos and files by time, capture location, device, and other dimensions',
  },
  'legal.privacy.s2l2': {
    'zh-CN': '批量重命名：基于模板批量修改文件名',
    'zh-TW': '批次重新命名：基於範本批次修改檔案名稱',
    en: 'Batch renaming: modifying file names in batches based on templates',
  },
  'legal.privacy.s2l3': {
    'zh-CN': '重复文件清理：基于哈希比对查找并清理重复文件',
    'zh-TW': '重複檔案清理：基於雜湊比對查找並清理重複檔案',
    en: 'Duplicate file cleanup: finding and removing duplicate files based on hash comparison',
  },
  'legal.privacy.s2l4': {
    'zh-CN': '系统垃圾清理：清理常见缓存与临时文件',
    'zh-TW': '系統垃圾清理：清理常見快取與暫存檔案',
    en: 'System junk cleanup: cleaning common caches and temporary files',
  },
  'legal.privacy.s2l5': {
    'zh-CN': '照片脱敏：清除照片中的 EXIF、GPS、相机信息、XMP 标记、Photoshop 信息等隐私数据',
    'zh-TW': '照片去識別化：清除照片中的 EXIF、GPS、相機資訊、XMP 標記、Photoshop 資訊等隱私資料',
    en: 'Photo sanitization: removing privacy data from photos, such as EXIF, GPS, camera information, XMP tags, and Photoshop metadata',
  },
  'legal.privacy.s2l6': {
    'zh-CN': '修复拍摄时间：为照片补写 EXIF 拍摄时间',
    'zh-TW': '修復拍攝時間：為照片補寫 EXIF 拍攝時間',
    en: 'Capture time repair: writing missing EXIF capture times back into photos',
  },
  'legal.privacy.s2l7': {
    'zh-CN': '写入地理信息：为照片补写 GPS 坐标',
    'zh-TW': '寫入地理資訊：為照片補寫 GPS 座標',
    en: 'Geolocation writing: writing missing GPS coordinates into photos',
  },
  'legal.privacy.s2l8': {
    'zh-CN': '操作日志：记录文件操作历史以供追溯',
    'zh-TW': '操作日誌：記錄檔案操作歷史以供追溯',
    en: 'Operation logs: recording file operation history for traceability',
  },
  'legal.privacy.s2p2': {
    'zh-CN': '本政策不适用于任何第三方网站、应用或服务。若您通过本应用内链接跳转至第三方页面（如开源仓库主页），请另行查阅其隐私政策。',
    'zh-TW': '本政策不適用於任何第三方網站、應用或服務。若您透過本應用內連結前往第三方頁面（如開源儲存庫首頁），請另行查閱其隱私權政策。',
    en: 'This Policy does not apply to any third-party websites, applications, or services. If you follow a link within the Application to a third-party page (such as the homepage of an open-source repository), please consult the privacy policy of that page separately.',
  },

  'legal.privacy.s3': {
    'zh-CN': '三、定义与术语',
    'zh-TW': '三、定義與術語',
    en: '3. Definitions and Terminology',
  },
  'legal.privacy.s3l1a': {
    'zh-CN': '个人信息',
    'zh-TW': '個人資訊',
    en: 'Personal Information',
  },
  'legal.privacy.s3l1b': {
    'zh-CN': '：以电子或其他方式记录的、能够单独或与其他信息结合识别特定自然人身份的各种信息。',
    'zh-TW': '：以電子或其他方式記錄的、能夠單獨或與其他資訊結合識別特定自然人身分的各種資訊。',
    en: ': any information recorded electronically or by other means that is capable of identifying the identity of a specific natural person, either alone or in combination with other information.',
  },
  'legal.privacy.s3l2a': {
    'zh-CN': '本地处理',
    'zh-TW': '本機處理',
    en: 'Local Processing',
  },
  'legal.privacy.s3l2b': {
    'zh-CN': '：全部数据处理过程均发生在您的设备上，不经过网络传输至任何远程服务器。',
    'zh-TW': '：全部資料處理過程均發生在您的裝置上，不經過網路傳輸至任何遠端伺服器。',
    en: ': all data processing takes place on your device, without any transmission over a network to any remote server.',
  },
  'legal.privacy.s3l3a': {
    'zh-CN': '本地存储',
    'zh-TW': '本機儲存',
    en: 'Local Storage',
  },
  'legal.privacy.s3l3b': {
    'zh-CN': '：数据保存在您设备上的用户目录中，开发者无法远程访问。',
    'zh-TW': '：資料保存在您裝置上的使用者目錄中，開發者無法遠端存取。',
    en: ': data is kept in the user directory on your device, and the Developer has no means of remote access to it.',
  },
  'legal.privacy.s3l4a': {
    'zh-CN': '操作日志',
    'zh-TW': '操作日誌',
    en: 'Operation Logs',
  },
  'legal.privacy.s3l4b': {
    'zh-CN': '：本应用在您执行文件操作时于本地生成的记录，包含操作类型、文件路径、处理结果等信息。',
    'zh-TW': '：本應用在您執行檔案操作時於本機產生的記錄，包含操作類型、檔案路徑、處理結果等資訊。',
    en: ': records generated locally by the Application when you perform file operations, containing information such as the operation type, file paths, and processing results.',
  },

  'legal.privacy.s4': {
    'zh-CN': '四、我不收集的信息（零收集承诺）',
    'zh-TW': '四、我不蒐集的資訊（零蒐集承諾）',
    en: '4. Information I Do Not Collect (Zero-Collection Commitment)',
  },
  'legal.privacy.s4p1': {
    'zh-CN': '本应用是一款纯离线运行的本地软件。我郑重承诺，本应用在设计与实现上不存在以下行为：',
    'zh-TW': '本應用是一款純離線運作的本機軟體。我鄭重承諾，本應用在設計與實作上不存在以下行為：',
    en: 'The Application is local software that runs entirely offline. I solemnly undertake that, in both its design and its implementation, the Application does not engage in any of the following:',
  },
  'legal.privacy.s4l1a': {
    'zh-CN': '不收集',
    'zh-TW': '不蒐集',
    en: 'Does not collect',
  },
  'legal.privacy.s4l1b': {
    'zh-CN': '您的姓名、电话、邮箱、地址、身份证件号码等任何个人身份信息',
    'zh-TW': '您的姓名、電話、電子郵件、地址、身分證件號碼等任何個人身分資訊',
    en: ' your name, telephone number, email address, postal address, identity card number, or any other personal identity information',
  },
  'legal.privacy.s4l2a': {
    'zh-CN': '不收集',
    'zh-TW': '不蒐集',
    en: 'Does not collect',
  },
  'legal.privacy.s4l2b': {
    'zh-CN': '您处理的文件、照片、视频的内容及其元数据',
    'zh-TW': '您處理的檔案、照片、影片的內容及其中繼資料',
    en: ' the contents of the files, photos, or videos you process, or their metadata',
  },
  'legal.privacy.s4l3a': {
    'zh-CN': '不收集',
    'zh-TW': '不蒐集',
    en: 'Does not collect',
  },
  'legal.privacy.s4l3b': {
    'zh-CN': '您的位置信息、通讯录、短信、通话记录、剪贴板内容、浏览记录',
    'zh-TW': '您的位置資訊、通訊錄、簡訊、通話記錄、剪貼簿內容、瀏覽記錄',
    en: ' your location information, contacts, SMS messages, call history, clipboard contents, or browsing history',
  },
  'legal.privacy.s4l4a': {
    'zh-CN': '不上传',
    'zh-TW': '不上傳',
    en: 'Does not upload',
  },
  'legal.privacy.s4l4b': {
    'zh-CN': '任何数据到开发者或任何第三方的远程服务器',
    'zh-TW': '任何資料到開發者或任何第三方的遠端伺服器',
    en: ' any data to any remote server belonging to the Developer or to any third party',
  },
  'legal.privacy.s4l5a': {
    'zh-CN': '不嵌入',
    'zh-TW': '不嵌入',
    en: 'Does not embed',
  },
  'legal.privacy.s4l5b': {
    'zh-CN': '任何统计 SDK、广告 SDK、数据埋点、用户行为分析或跟踪组件',
    'zh-TW': '任何統計 SDK、廣告 SDK、資料埋點、使用者行為分析或追蹤元件',
    en: ' any analytics SDK, advertising SDK, data tracking beacon, user behavior analysis, or tracking component',
  },
  'legal.privacy.s4l6a': {
    'zh-CN': '不进行',
    'zh-TW': '不進行',
    en: 'Does not perform',
  },
  'legal.privacy.s4l6b': {
    'zh-CN': '任何形式的用户画像、个性化推荐或精准营销',
    'zh-TW': '任何形式的使用者輪廓、個人化推薦或精準行銷',
    en: ' any form of user profiling, personalized recommendation, or targeted marketing',
  },
  'legal.privacy.s4p2': {
    'zh-CN': '由于本应用根本不收集您的任何数据，您无需向我提供任何个人信息即可完整使用本应用的全部功能。',
    'zh-TW': '由於本應用根本不蒐集您的任何資料，您無需向我提供任何個人資訊即可完整使用本應用的全部功能。',
    en: 'Because the Application does not collect any of your data whatsoever, you can use the full functionality of the Application without providing me with any personal information.',
  },

  'legal.privacy.s5': {
    'zh-CN': '五、本应用处理的数据及处理方式',
    'zh-TW': '五、本應用處理的資料及處理方式',
    en: '5. Data Processed by the Application and Methods of Processing',
  },
  'legal.privacy.s5p1': {
    'zh-CN': '本应用在您主动操作时，会在您的设备本地读写以下数据。全部处理过程不经过网络，开发者无法获知处理内容：',
    'zh-TW': '本應用在您主動操作時，會在您的裝置本機讀寫以下資料。全部處理過程不經過網路，開發者無法得知處理內容：',
    en: 'When you actively operate the Application, it reads and writes the following data locally on your device. The entire processing takes place without any involvement of the network, and the Developer cannot learn the contents being processed:',
  },
  'legal.privacy.s5l1a': {
    'zh-CN': '文件与照片数据',
    'zh-TW': '檔案與照片資料',
    en: 'File and Photo Data',
  },
  'legal.privacy.s5l1b': {
    'zh-CN': '：仅在您明确选择待处理目录、选择输出目录并点击执行后，本应用才在本地读取相应文件，进行整理、重命名、删除、元数据修改（如 EXIF 拍摄时间、GPS 坐标、隐私信息清除）等操作。处理结果写入您指定的本地输出目录，源文件默认保留',
    'zh-TW': '：僅在您明確選擇待處理目錄、選擇輸出目錄並點擊執行後，本應用才在本機讀取相應檔案，進行整理、重新命名、刪除、中繼資料修改（如 EXIF 拍攝時間、GPS 座標、隱私資訊清除）等操作。處理結果寫入您指定的本機輸出目錄，來源檔案預設保留',
    en: ': only after you explicitly select the directory to be processed, select an output directory, and click to execute will the Application read the corresponding files locally and perform operations such as organizing, renaming, deleting, and metadata modification (such as EXIF capture time, GPS coordinates, and removal of privacy information). The results are written to the local output directory you specify, and source files are retained by default',
  },
  'legal.privacy.s5l2a': {
    'zh-CN': '目录与界面偏好设置',
    'zh-TW': '目錄與介面偏好設定',
    en: 'Directory and Interface Preferences',
  },
  'legal.privacy.s5l2b': {
    'zh-CN': '：您选择的工作目录、输出目录、主题模式、窗口行为等设置，仅以本地配置形式保存在您设备的本地存储中，用于在您下次打开应用时恢复使用习惯',
    'zh-TW': '：您選擇的工作目錄、輸出目錄、主題模式、視窗行為等設定，僅以本機設定檔形式保存在您裝置的本機儲存中，用於在您下次開啟應用時恢復使用習慣',
    en: ': settings such as the working directory, output directory, theme mode, and window behavior that you select are saved only as local configuration in the local storage on your device, and are used to restore your preferences the next time you open the Application',
  },
  'legal.privacy.s5l3a': {
    'zh-CN': '操作日志',
    'zh-TW': '操作日誌',
    en: 'Operation Logs',
  },
  'legal.privacy.s5l3b': {
    'zh-CN': '：为便于您追溯、复核已执行的操作，本应用会在您设备的本地日志目录生成操作记录（操作类型、文件路径、处理结果、时间）。日志默认按您设置的保留天数自动清理，您也可随时在应用内删除单条日志或清空全部日志',
    'zh-TW': '：為便於您追溯、複核已執行的操作，本應用會在您裝置的本機日誌目錄產生操作記錄（操作類型、檔案路徑、處理結果、時間）。日誌預設依您設定的保留天數自動清理，您也可隨時在應用內刪除單筆日誌或清空全部日誌',
    en: ': to allow you to trace and review operations that have been performed, the Application generates operation records (operation type, file paths, processing results, and time) in the local log directory on your device. Logs are automatically cleaned up by default according to the retention period you configure, and you may also delete individual log entries or clear all logs at any time within the Application',
  },
  'legal.privacy.s5l4a': {
    'zh-CN': '离线地理数据',
    'zh-TW': '離線地理資料',
    en: 'Offline Geographic Data',
  },
  'legal.privacy.s5l4b': {
    'zh-CN': '：写入地理信息功能使用内置的离线地名数据库（仅包含公开的行政区划与地名数据），经纬度与地名的匹配计算完全在本地完成，不请求任何在线地图或定位服务',
    'zh-TW': '：寫入地理資訊功能使用內建的離線地名資料庫（僅包含公開的行政區劃與地名資料），經緯度與地名的匹配計算完全在本機完成，不請求任何線上地圖或定位服務',
    en: ': the geolocation writing feature uses a built-in offline place-name database (containing only publicly available administrative divisions and place-name data); the matching of latitude and longitude to place names is computed entirely locally, without requesting any online map or location service',
  },

  'legal.privacy.s6': {
    'zh-CN': '六、系统权限使用说明',
    'zh-TW': '六、系統權限使用說明',
    en: '6. System Permissions',
  },
  'legal.privacy.s6p1': {
    'zh-CN': '为实现文件管理功能，本应用需要向您申请以下系统权限：',
    'zh-TW': '為實現檔案管理功能，本應用需要向您申請以下系統權限：',
    en: 'To provide file management functionality, the Application requires the following system permissions:',
  },
  'legal.privacy.s6l1a': {
    'zh-CN': '文件系统访问权限',
    'zh-TW': '檔案系統存取權限',
    en: 'File System Access Permission',
  },
  'legal.privacy.s6l1b': {
    'zh-CN': '：仅用于读取您明确选择的目录中的文件、向您明确指定的输出目录写入文件。本应用不会在您未主动操作时扫描或访问任何其他目录',
    'zh-TW': '：僅用於讀取您明確選擇的目錄中的檔案、向您明確指定的輸出目錄寫入檔案。本應用不會在您未主動操作時掃描或存取任何其他目錄',
    en: ': used solely to read files in the directories you explicitly select and to write files to the output directory you explicitly designate. The Application will not scan or access any other directory when you have not actively initiated an operation',
  },
  'legal.privacy.s6p2': {
    'zh-CN': '上述权限的使用范围以您在应用内主动选择的目录为限。您随时可以通过不再选择相应目录的方式，停止本应用对该目录的访问。',
    'zh-TW': '上述權限的使用範圍以您在應用內主動選擇的目錄為限。您隨時可以透過不再選擇相應目錄的方式，停止本應用對該目錄的存取。',
    en: 'The scope of the above permissions is limited to the directories you actively select within the Application. You may at any time stop the access of the Application to a directory simply by no longer selecting that directory.',
  },

  'legal.privacy.s7': {
    'zh-CN': '七、网络使用说明',
    'zh-TW': '七、網路使用說明',
    en: '7. Network Usage',
  },
  'legal.privacy.s7p1': {
    'zh-CN': '本应用的全部核心功能均为离线运行，不依赖网络连接。应用本身不内置自动更新通道，不请求任何远程资源。仅在以下与本应用功能无直接关联的情形下，可能由您的操作间接触发网络访问：',
    'zh-TW': '本應用的全部核心功能均為離線運作，不依賴網路連線。應用本身不內建自動更新通道，不請求任何遠端資源。僅在以下與本應用功能無直接關聯的情形下，可能由您的操作間接觸發網路存取：',
    en: 'All core functionality of the Application runs offline and does not depend on a network connection. The Application itself contains no built-in automatic update channel and requests no remote resources. Only in the following situations, which are not directly related to the functionality of the Application, may network access be indirectly triggered by your actions:',
  },
  'legal.privacy.s7l1': {
    'zh-CN': '您主动点击应用内指向外部网站（如开发者主页）的链接，由您的浏览器发起访问',
    'zh-TW': '您主動點擊應用內指向外部網站（如開發者首頁）的連結，由您的瀏覽器發起存取',
    en: 'you actively click a link within the Application pointing to an external website (such as the homepage of the Developer), and the access is initiated by your browser',
  },
  'legal.privacy.s7l2': {
    'zh-CN': '您通过应用商店等分发渠道安装或更新本应用，相关平台按其自身的隐私政策运行',
    'zh-TW': '您透過應用程式商店等發行管道安裝或更新本應用，相關平台依其自身的隱私權政策運作',
    en: 'you install or update the Application through an app store or other distribution channel; the relevant platform operates under its own privacy policy',
  },

  'legal.privacy.s8': {
    'zh-CN': '八、数据的存储地点、期限与删除',
    'zh-TW': '八、資料的儲存地點、期限與刪除',
    en: '8. Storage Location, Retention Period, and Deletion of Data',
  },
  'legal.privacy.s8l1a': {
    'zh-CN': '存储地点',
    'zh-TW': '儲存地點',
    en: 'Storage Location',
  },
  'legal.privacy.s8l1b': {
    'zh-CN': '：本应用产生的全部数据（配置文件、操作日志）均存储在您本机的用户目录下，不存在任何云端副本，开发者无法远程访问、获取或转移',
    'zh-TW': '：本應用產生的全部資料（設定檔、操作日誌）均儲存在您本機的使用者目錄下，不存在任何雲端副本，開發者無法遠端存取、取得或轉移',
    en: ': all data generated by the Application (configuration files and operation logs) is stored in the user directory on your local machine; no cloud copy exists, and the Developer cannot remotely access, obtain, or transfer it',
  },
  'legal.privacy.s8l2a': {
    'zh-CN': '存储期限',
    'zh-TW': '儲存期限',
    en: 'Retention Period',
  },
  'legal.privacy.s8l2b': {
    'zh-CN': '：配置文件在本应用安装期间持续保存；操作日志按您在设置中选择的保留天数自动清理',
    'zh-TW': '：設定檔在本應用安裝期間持續保存；操作日誌依您在設定中選擇的保留天數自動清理',
    en: ': configuration files persist for as long as the Application is installed; operation logs are automatically cleaned up according to the retention period you select in the settings',
  },
  'legal.privacy.s8l3a': {
    'zh-CN': '主动删除',
    'zh-TW': '主動刪除',
    en: 'Active Deletion',
  },
  'legal.privacy.s8l3b': {
    'zh-CN': '：您可在应用内随时删除单条日志或清空全部日志；卸载本应用并手动删除其数据目录后，全部本地数据即被彻底移除',
    'zh-TW': '：您可在應用內隨時刪除單筆日誌或清空全部日誌；解除安裝本應用並手動刪除其資料目錄後，全部本機資料即被徹底移除',
    en: ': you may delete individual log entries or clear all logs at any time within the Application; after you uninstall the Application and manually delete its data directory, all local data is completely removed',
  },

  'legal.privacy.s9': {
    'zh-CN': '九、数据安全保护措施',
    'zh-TW': '九、資料安全保護措施',
    en: '9. Data Security Safeguards',
  },
  'legal.privacy.s9p1': {
    'zh-CN': '我通过产品设计为您的数据安全提供多重保障：',
    'zh-TW': '我透過產品設計為您的資料安全提供多重保障：',
    en: 'I provide multiple safeguards for the security of your data through the design of the product:',
  },
  'legal.privacy.s9l1': {
    'zh-CN': '所有文件操作均需您在界面上明确选择目录并主动点击执行，不存在后台静默操作',
    'zh-TW': '所有檔案操作均需您在介面上明確選擇目錄並主動點擊執行，不存在背景靜默操作',
    en: 'every file operation requires you to explicitly select directories in the interface and actively click to execute; there are no silent background operations',
  },
  'legal.privacy.s9l2': {
    'zh-CN': '整理、脱敏、修复等破坏性风险较高的操作默认将结果写入独立的输出目录，源文件保持原样',
    'zh-TW': '整理、去識別化、修復等破壞性風險較高的操作預設將結果寫入獨立的輸出目錄，來源檔案保持原樣',
    en: 'operations carrying a higher risk of destruction, such as organizing, sanitization, and repair, write their results to a separate output directory by default, leaving source files untouched',
  },
  'legal.privacy.s9l3': {
    'zh-CN': '每一步操作均生成详细的本地日志，方便您追溯和复核',
    'zh-TW': '每一步操作均產生詳細的本機日誌，方便您追溯和複核',
    en: 'every operation step generates a detailed local log so that you can trace and review it',
  },
  'legal.privacy.s9l4': {
    'zh-CN': '应用代码以开源形式发布，处理逻辑可被公开审查',
    'zh-TW': '應用程式碼以開源形式發布，處理邏輯可被公開審查',
    en: 'the source code of the Application is released as open source, and its processing logic is open to public review',
  },
  'legal.privacy.s9p2': {
    'zh-CN': '同时请您知悉，本地存储的数据安全亦取决于您设备本身的安全状况（如磁盘加密、账户密码等），请您妥善保管设备。',
    'zh-TW': '同時請您知悉，本機儲存的資料安全亦取決於您裝置本身的安全狀況（如磁碟加密、帳戶密碼等），請您妥善保管裝置。',
    en: 'Please also be aware that the security of locally stored data also depends on the security of your device itself (such as disk encryption and account passwords); please keep your device properly secured.',
  },

  'legal.privacy.s10': {
    'zh-CN': '十、未成年人保护',
    'zh-TW': '十、未成年人保護',
    en: '10. Protection of Minors',
  },
  'legal.privacy.s10p1': {
    'zh-CN': '本应用面向一般用户提供，不以未成年人为特定服务对象，亦不会以任何方式收集未成年人个人信息。若未成年人使用本软件，应在监护人的指导下进行，并由监护人帮助判断操作的适当性。',
    'zh-TW': '本應用針對一般使用者提供，不以未成年人為特定服務對象，亦不會以任何方式蒐集未成年人個人資訊。若未成年人使用本軟體，應在監護人的指導下進行，並由監護人協助判斷操作的適當性。',
    en: 'The Application is provided to general users; it does not target minors as a specific service audience and will not collect personal information of minors in any manner. If a minor uses this software, such use should take place under the guidance of a guardian, with the guardian assisting in judging the appropriateness of the operations.',
  },

  'legal.privacy.s11': {
    'zh-CN': '十一、您的权利',
    'zh-TW': '十一、您的權利',
    en: '11. Your Rights',
  },
  'legal.privacy.s11p1': {
    'zh-CN': '您对自己的全部数据享有完整、唯一的控制权：',
    'zh-TW': '您對自己的全部資料享有完整、唯一的控制權：',
    en: 'You hold complete and exclusive control over all of your data:',
  },
  'legal.privacy.s11l1a': {
    'zh-CN': '知情权',
    'zh-TW': '知情權',
    en: 'Right to Be Informed',
  },
  'legal.privacy.s11l1b': {
    'zh-CN': '：本政策完整披露了应用处理数据的全部方式，且代码开源可供查验',
    'zh-TW': '：本政策完整揭露了應用處理資料的全部方式，且程式碼開源可供查驗',
    en: ': this Policy fully discloses all ways in which the Application processes data, and the source code is open source and available for inspection',
  },
  'legal.privacy.s11l2a': {
    'zh-CN': '控制权',
    'zh-TW': '控制權',
    en: 'Right of Control',
  },
  'legal.privacy.s11l2b': {
    'zh-CN': '：您可以随时查看、修改、删除本应用处理的任何文件',
    'zh-TW': '：您可以隨時查看、修改、刪除本應用處理的任何檔案',
    en: ': you may view, modify, or delete any file processed by the Application at any time',
  },
  'legal.privacy.s11l3a': {
    'zh-CN': '删除权',
    'zh-TW': '刪除權',
    en: 'Right to Erasure',
  },
  'legal.privacy.s11l3b': {
    'zh-CN': '：您可以随时删除单条操作日志或清空全部日志，可以随时卸载应用并删除残留数据',
    'zh-TW': '：您可以隨時刪除單筆操作日誌或清空全部日誌，可以隨時解除安裝應用並刪除殘留資料',
    en: ': you may delete individual operation logs or clear all logs at any time, and you may uninstall the Application and delete residual data at any time',
  },
  'legal.privacy.s11p2': {
    'zh-CN': '由于开发者不收集您的任何数据，因此不存在"撤回授权""注销账号""导出/转移数据""拒绝个性化推荐"等需求场景；如您认为本应用存在未在本政策中披露的数据处理行为，欢迎通过第十六章提供的联系方式向我举报，我将及时核实并处理。',
    'zh-TW': '由於開發者不蒐集您的任何資料，因此不存在「撤回授權」「註銷帳號」「匯出/轉移資料」「拒絕個人化推薦」等需求場景；如您認為本應用存在未在本政策中揭露的資料處理行為，歡迎透過第十六章提供的聯絡方式向我檢舉，我將及時核實並處理。',
    en: 'Because the Developer does not collect any of your data, there are no scenarios such as "withdrawing consent", "deactivating an account", "exporting or transferring data", or "declining personalized recommendations"; if you believe that the Application engages in any data processing not disclosed in this Policy, you are welcome to report it to me via the contact information provided in Chapter 16, and I will verify and address it in a timely manner.',
  },

  'legal.privacy.s12': {
    'zh-CN': '十二、第三方组件说明',
    'zh-TW': '十二、第三方元件說明',
    en: '12. Third-Party Components',
  },
  'legal.privacy.s12p1': {
    'zh-CN': '本应用基于 Tauri、Vue 3、Element Plus、Rust 生态等开源组件构建。这些组件作为应用的一部分在您的本地运行，其数据处理行为同样受本政策"零收集承诺"约束。各组件的许可证信息详见应用内《许可协议》。',
    'zh-TW': '本應用基於 Tauri、Vue 3、Element Plus、Rust 生態系等開源元件建構。這些元件作為應用的一部分在您的本機運作，其資料處理行為同樣受本政策「零蒐集承諾」約束。各元件的授權條款資訊詳見應用內《授權協議》。',
    en: 'The Application is built upon open-source components such as Tauri, Vue 3, Element Plus, and the Rust ecosystem. These components run locally as part of the Application, and their data processing activities are likewise bound by the "Zero-Collection Commitment" of this Policy. License information for each component can be found in the License Agreement within the Application.',
  },

  'legal.privacy.s13': {
    'zh-CN': '十三、免责与风险提示',
    'zh-TW': '十三、免責與風險提示',
    en: '13. Disclaimer and Risk Notice',
  },
  'legal.privacy.s13p1': {
    'zh-CN': '文件操作具有一定固有风险。尽管本应用在设计中默认保留源文件并记录操作日志，您仍应注意：',
    'zh-TW': '檔案操作具有一定固有風險。儘管本應用在設計中預設保留來源檔案並記錄操作日誌，您仍應注意：',
    en: 'File operations carry certain inherent risks. Although the Application is designed to retain source files and record operation logs by default, you should still take note of the following:',
  },
  'legal.privacy.s13l1': {
    'zh-CN': '批量操作前请确认已选定正确的目录与输出位置',
    'zh-TW': '批次操作前請確認已選定正確的目錄與輸出位置',
    en: 'before batch operations, please confirm that the correct directories and output location have been selected',
  },
  'legal.privacy.s13l2': {
    'zh-CN': '重要数据请自行备份；因您自身误操作、磁盘故障、系统异常等非软件缺陷原因导致的数据损失，由您自行承担',
    'zh-TW': '重要資料請自行備份；因您自身誤操作、磁碟故障、系統異常等非軟體缺陷原因導致的資料損失，由您自行承擔',
    en: 'please back up important data on your own; any loss of data caused by your own misoperation, disk failure, system anomalies, or other reasons not attributable to software defects shall be borne by you',
  },
  'legal.privacy.s13l3': {
    'zh-CN': '请通过正规渠道下载本应用，使用来源不明的修改版造成的问题不在保障范围内',
    'zh-TW': '請透過正規管道下載本應用，使用來源不明的修改版造成的問題不在保障範圍內',
    en: 'please download the Application through official channels; problems caused by modified versions of unknown origin are not covered by this guarantee',
  },
  'legal.privacy.s13p2': {
    'zh-CN': '本软件按"现状"提供，不作任何明示或默示的担保，我不保证软件完全无缺陷。您确认已知悉文件批量操作存在固有风险，并承诺在使用本软件处理重要数据前自行完成备份。在法律允许的最大范围内，对因使用本软件导致的文件或数据丢失、损坏（包括但不限于数据丢失、数据损坏、业务中断、预期利益损失等一切直接或间接损失），无论基于何种原因（包括软件缺陷），均不由我承担。仅法律强制规定不得免除的责任除外。详细的责任划分请参阅《许可协议》。',
    'zh-TW': '本軟體按「現狀」提供，不作任何明示或默示的擔保，我不保證軟體完全無缺陷。您確認已知悉檔案批次操作存在固有風險，並承諾在使用本軟體處理重要資料前自行完成備份。在法律允許的最大範圍內，對因使用本軟體導致的檔案或資料遺失、損壞（包括但不限於資料遺失、資料損壞、業務中斷、預期利益損失等一切直接或間接損失），無論基於何種原因（包括軟體缺陷），均不由我承擔。僅法律強制規定不得免除的責任除外。詳細的責任劃分請參閱《授權協議》。',
    en: 'This Software is provided "as is" without warranty of any kind, express or implied, and I do not guarantee that it is entirely defect-free. You acknowledge that you are aware of the inherent risks of batch file operations and undertake to back up your important data before processing it with this Software. To the maximum extent permitted by law, I shall not be liable for any loss or corruption of files or data arising from the use of this Software (including, but not limited to, data loss, data corruption, business interruption, loss of anticipated benefits, and any other direct or indirect losses), regardless of the cause (including software defects). Exceptions apply only where liability may not be excluded under mandatory provisions of law. For the detailed allocation of liability, please refer to the License Agreement.',
  },

  'legal.privacy.s14': {
    'zh-CN': '十四、政策的更新与生效',
    'zh-TW': '十四、政策的更新與生效',
    en: '14. Updates and Effectiveness of this Policy',
  },
  'legal.privacy.s14p1': {
    'zh-CN': '本政策可能随应用功能演进而更新。更新后的政策将在应用内公布并标注更新日期，重大变更将以显著方式提示。您可以在设置页面的"法律信息"入口随时查阅最新版本。',
    'zh-TW': '本政策可能隨應用功能演進而更新。更新後的政策將在應用內公布並標註更新日期，重大變更將以顯著方式提示。您可以在設定頁面的「法律資訊」入口隨時查閱最新版本。',
    en: 'This Policy may be updated as the features of the Application evolve. The updated Policy will be published within the Application with the update date noted, and material changes will be brought to your attention in a prominent manner. You may consult the latest version at any time via the "Legal Information" entry on the settings page.',
  },
  'legal.privacy.s14p2': {
    'zh-CN': '若您在政策更新后继续使用本应用，即视为您已阅读并同意接受更新后的政策；若您不同意更新后的内容，请停止使用本应用。',
    'zh-TW': '若您在政策更新後繼續使用本應用，即視為您已閱讀並同意接受更新後的政策；若您不同意更新後的內容，請停止使用本應用。',
    en: 'If you continue to use the Application after this Policy has been updated, you shall be deemed to have read and agreed to accept the updated Policy; if you do not agree to the updated contents, please stop using the Application.',
  },

  'legal.privacy.s15': {
    'zh-CN': '十五、争议解决',
    'zh-TW': '十五、爭議解決',
    en: '15. Dispute Resolution',
  },
  'legal.privacy.s15p1': {
    'zh-CN': '因本政策产生的或与本政策相关的任何争议，双方应首先友好协商解决；协商不成的，可依照《许可协议》中约定的争议解决方式处理。本政策的订立、效力、解释与执行均适用中华人民共和国法律。',
    'zh-TW': '因本政策產生的或與本政策相關的任何爭議，雙方應首先友好協商解決；協商不成的，可依照《授權協議》中約定的爭議解決方式處理。本政策的訂立、效力、解釋與執行均適用中華人民共和國法律。',
    en: 'Any dispute arising out of or in connection with this Policy shall first be resolved through friendly negotiation between the parties; if such negotiation fails, the dispute may be handled in accordance with the dispute resolution method stipulated in the License Agreement. The conclusion, validity, interpretation, and enforcement of this Policy shall all be governed by the laws of the People\u2019s Republic of China.',
  },

  'legal.privacy.s16': {
    'zh-CN': '十六、如何联系我',
    'zh-TW': '十六、如何聯絡我',
    en: '16. How to Contact Me',
  },
  'legal.privacy.s16p1': {
    'zh-CN': '如您对本政策或本应用的数据处理方式有任何疑问、意见、建议或举报，可通过以下方式联系开发者：',
    'zh-TW': '如您對本政策或本應用的資料處理方式有任何疑問、意見、建議或檢舉，可透過以下方式聯絡開發者：',
    en: 'If you have any questions, comments, suggestions, or reports regarding this Policy or the data processing practices of the Application, you may contact the Developer through the following channels:',
  },
  'legal.privacy.s16l1': {
    'zh-CN': '开发者：杨圣洲',
    'zh-TW': '開發者：楊聖洲',
    en: 'Developer: 杨圣洲',
  },
  'legal.privacy.s16l2': {
    'zh-CN': '邮箱：YangSZ03@foxmail.com',
    'zh-TW': '電子郵件：YangSZ03@foxmail.com',
    en: 'Email: YangSZ03@foxmail.com',
  },
  'legal.privacy.s16l3': {
    'zh-CN': '主页：https://github.com/YangShengzhou03',
    'zh-TW': '首頁：https://github.com/YangShengzhou03',
    en: 'Homepage: https://github.com/YangShengzhou03',
  },
  'legal.privacy.s16p2': {
    'zh-CN': '我将在收到您的反馈后尽快予以核实和处理，一般情况下于十五个工作日内回复。',
    'zh-TW': '我將在收到您的回饋後儘快予以核實和處理，一般情況下於十五個工作日內回覆。',
    en: 'I will verify and address your feedback as soon as possible after receiving it and will, as a general rule, reply within fifteen business days.',
  },

  'legal.privacy.s17': {
    'zh-CN': '十七、权利保留',
    'zh-TW': '十七、權利保留',
    en: '17. Reservation of Rights',
  },
  'legal.privacy.s17p1': {
    'zh-CN': '本应用以本地处理为核心设计，我不会主动收集、上传或共享您的任何文件内容和个人信息；如未来因法律法规强制要求或提供新功能所必需而需要调整数据处理方式的，我将通过应用内公告或版本更新说明提前告知，并再次征求您的同意。',
    'zh-TW': '本應用以本地處理為核心設計，我不會主動收集、上傳或共用您的任何檔案內容和個人資訊；如未來因法律法規強制要求或提供新功能所必需而需要調整資料處理方式的，我將透過應用內公告或版本更新說明提前告知，並再次徵求您的同意。',
    en: 'This Application is designed around local processing. I do not actively collect, upload, or share any of your file content or personal information. If, in the future, mandatory legal requirements or new features make adjustments to data processing necessary, I will notify you in advance through in-app announcements or release notes and seek your consent again.',
  },
  'legal.privacy.s17p2': {
    'zh-CN': '在法律允许的最大范围内，我保留对本政策进行更新、修订和解释的权利；重大变更将更新"更新日期"并标注适用版本。本政策未明确规定的事项，参照《许可协议》相关条款处理。本政策不赋予任何第三方任何合同权利或救济。',
    'zh-TW': '在法律允許的最大範圍內，我保留對本政策進行更新、修訂和解釋的權利；重大變更將更新「更新日期」並標註適用版本。本政策未明確規定之事項，參照《授權協議》相關條款處理。本政策不賦予任何第三方任何合約權利或救濟。',
    en: 'To the maximum extent permitted by law, I reserve the right to update, revise, and interpret this Policy; material changes will update the "Last updated" date and the applicable version. Matters not expressly addressed in this Policy shall be handled with reference to the relevant provisions of the License Agreement. This Policy confers no contractual rights or remedies upon any third party.',
  },
}

export default dict
