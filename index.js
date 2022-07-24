import ajax from './ajax'
/**
 * 重要说明！！！
 * 因为，后台已对「/backend，/frontend，/files」接口代理,页面路由绝对禁止出现/backend、/frontend、/files（远景包括map）
 * 在定义接口代理时，上述的路由单词已经被定义，如果使用，刷新页面将出现404，
 * @type {string}
 */

// 后台api接口
let backendAPI = '/backend';

// 登录接口
export const requestLogin = params => ajax(`${backendAPI}/login/web`, params, 'POST');
// {{host}}:{{port}}/login

// 注销接口
export const requestLogout = params => ajax(`${backendAPI}/logout`, params, 'POST');
// {{host}}:{{port}}/logout

// 获取日志接口
export const getLogList = params => ajax(`${backendAPI}/api/set/log`, params, 'POST');
// {{host}}:{{port}}/backend/system/log/page

// 获取日志类别接口
export const getLogType = params => ajax(`${backendAPI}/api/set/logtype`, params, 'POST');
// {{host}}:{{port}}/backend/system/log/type

// 导出日志
export const downloadLogExcel = `${backendAPI}/api/set/log/excel`;

// 上传头像
export const uploadLogo = params => ajax(`${backendAPI}/api/set/uploadlogo`, params, 'POST');
// {{host}}:{{port}}/backend/system/user/logo

// 获取个人信息
export const getPersonal = params => ajax(`${backendAPI}/api/set/personal`, params, 'GET');
// {{host}}:{{port}}/backend/system/user

// 修改密码
export const setPassword = params => ajax(`${backendAPI}/api/set/password`, params, 'PUT');
// {{host}}:{{port}}/backend/system/user/password

// 修改用户信息
export const setUserInfo = params => ajax(`${backendAPI}/api/set/update`, params, 'PUT');
// {{host}}:{{port}}/backend/system/user


// 上传笔记、消息图片
export const uploadNewsPicture = `${backendAPI}/api/oss/picture/illustrated`;
// {{host}}:{{port}}/backend/oss/picture/base64

// 获取动态
export const getNewsList = params => ajax(`${backendAPI}/api/message/news`, params, 'GET');
//{{host}}:{{port}}/backend/content/news

// 发布动态
export const publishNews = params => ajax(`${backendAPI}/api/message/news/publish`, params, 'POST');
// {{host}}:{{port}}/backend/content/news

// 删除动态
export const deleteNews = params => ajax(`${backendAPI}/api/message/news/delete`, params, 'DELETE');
// {{host}}:{{port}}/backend/content/news/15

// 查询动态
export const getNews = params => ajax(`${backendAPI}/api/message/news/show`, params, 'GET');
// {{host}}:{{port}}/backend/content/news/15

// 修改动态
export const editNews = params => ajax(`${backendAPI}/api/message/news/edit`, params, 'PUT');
// {{host}}:{{port}}/backend/content/news

// 查看分页后的图片
export const getPictureList = params => ajax(`${backendAPI}/api/oss/picture`, params, 'GET');
// {{host}}:{{port}}/backend/oss/picture/page

// 上传壁纸
export const uploadWallpaper = `${backendAPI}/api/oss/picture/wallpaper`;
// {{host}}:{{port}}/backend/oss/picture/file

// 删除壁纸/插图
export const deletePicture = params => ajax(`${backendAPI}/api/oss/picture/delete`, params, 'DELETE');
// {{host}}:{{port}}/backend/oss/picture/22

// 上传文件
export const uploadFile = `${backendAPI}/api/oss/files/upload`;
// {{host}}:{{port}}/backend/oss/files/file

// 查看分页后的文件
export const getFileList = params => ajax(`${backendAPI}/api/oss/files`, params, 'GET');
// 删除文件
export const deleteFile = params => ajax(`${backendAPI}/api/oss/files/delete`, params, 'DELETE');
// 修改文件
export const editFile = params => ajax(`${backendAPI}/api/oss/files/edit`, params, 'PUT');
// 下载文件
export const downloadFileForAdmin = `${backendAPI}/api/oss/files/download/`;
// 创建笔记簿
export const createNoteBook = params => ajax(`${backendAPI}/api/message/notebook/create`, params, 'POST');
// 修改笔记簿
export const updateNoteBook = params => ajax(`${backendAPI}/api/message/notebook/edit`, params, 'PUT');
// 删除笔记簿
export const deleteNoteBook = params => ajax(`${backendAPI}/api/message/notebook/delete`, params, 'DELETE');
// 获取笔记簿列表
export const getNoteBookList = params => ajax(`${backendAPI}/api/message/notebook`, params, 'GET');
// 获取笔记簿
export const getNoteBookGroup = params => ajax(`${backendAPI}/api/message/notebook/group`, params, 'GET');
// 查询单条笔记簿
export const getNoteBook = params => ajax(`${backendAPI}/api/message/notebook/show`, params, 'GET');
// 创建笔记
export const createNotes = params => ajax(`${backendAPI}/api/message/notes/create`, params, 'POST');
// 修改笔记
export const updateNotes = params => ajax(`${backendAPI}/api/message/notes/edit`, params, 'PUT');
// 删除笔记
export const deleteNotes = params => ajax(`${backendAPI}/api/message/notes/delete`, params, 'DELETE');
// 获取笔记
export const getNotesList = params => ajax(`${backendAPI}/api/message/notes`, params, 'GET');
// 查询单条笔记
export const getNotes = params => ajax(`${backendAPI}/api/message/notes/show`, params, 'GET');
// 获取该月计划
export const getPlanList = params => ajax(`${backendAPI}/api/set/plan`, params, 'GET');
// 添加计划
export const createPlan = params => ajax(`${backendAPI}/api/set/plan/create`, params, 'POST');
// 修改计划
export const updatePlan = params => ajax(`${backendAPI}/api/set/plan/edit`, params, 'PUT');
// 删除计划
export const deletePlan = params => ajax(`${backendAPI}/api/set/plan/delete`, params, 'DELETE');
// 获取所有的支付类别
export const getFinancialType = params => ajax(`${backendAPI}/api/financial/transactionType`, params, 'GET');
// 获取所有的交易摘要
export const getFinancialAmount = params => ajax(`${backendAPI}/api/financial/transactionAmount`, params, 'GET');
// 获取财政流水
export const getTransactionList = params => ajax(`${backendAPI}/api/financial/transaction`, params, 'GET');
// 查看收支明细（明细记录折叠存）
export const getTransactionDetail = params => ajax(`${backendAPI}/api/financial/transactionDetail`, params, 'GET');
// 分页查看收支明细（明细记录折叠存）
export const getTransactionDetailPage = params => ajax(`${backendAPI}/api/financial/transactionDetailPage`, params, 'GET');
// 财政申报
export const applyTransaction = params => ajax(`${backendAPI}/api/financial/insertTransaction`, params, 'POST');
// 修改流水
export const updateTransaction = params => ajax(`${backendAPI}/api/financial/updateTransaction`, params, 'PUT');
// 删除流水
export const deleteTransaction = params => ajax(`${backendAPI}/api/financial/deleteTransaction`, params, 'DELETE');
// 导出流水
export const downTransaction = `${backendAPI}/api/financial/outTransactionListExcel`;
// 导出流水明细
export const outTransactionInfoExcel = `${backendAPI}/api/financial/outTransactionInfoExcel`;
// 获取流水明细
export const getTransactionInfo = params => ajax(`${backendAPI}/api/financial/transactionInfo`, params, 'GET');
// 添加流水明细
export const insertTransactioninfo = params => ajax(`${backendAPI}/api/financial/insertTransactioninfo`, params, 'POST');
// 修改流水明细
export const updateTransactioninfo = params => ajax(`${backendAPI}/api/financial/updateTransactioninfo`, params, 'PUT');
// 删除流水明细
export const deleteTransactioninfo = params => ajax(`${backendAPI}/api/financial/deleteTransactioninfo`, params, 'DELETE');
// 按天统计流水
export const totalTransactionForDay = params => ajax(`${backendAPI}/api/financial/totalTransactionForDay`, params, 'GET');
// 导出按天统计的报表
export const outTransactionForDayExcel = `${backendAPI}/api/financial/outTransactionForDayExcel`;
// 按月统计流水
export const totalTransactionForMonth = params => ajax(`${backendAPI}/api/financial/totalTransactionForMonth`, params, 'GET');
// 导出按月统计的报表
export const outTransactionForMonthExcel = `${backendAPI}/api/financial/outTransactionForMonthExcel`;
// 按年统计流水
export const totalTransactionForYear = params => ajax(`${backendAPI}/api/financial/totalTransactionForYear`, params, 'GET');
// 导出按月统计的报表
export const outTransactionForYearExcel = `${backendAPI}/api/financial/outTransactionForYearExcel`;
// 查看数据库备份执行列表
export const getBackUpDBList = params => ajax(`${backendAPI}/api/oss/db`, params, 'GET');
// 下载备份的数据库文件
export const downloadBackUpDB = `${backendAPI}/api/oss/db/download/`;
// 获取数据总量及词云数据
export const getCountAndWordCloud = () => ajax(`${backendAPI}/api/set/countAndWordCloud`, {}, 'GET');
// 查询活跃度
export const getActivityRate= params => ajax(`${backendAPI}/api/set/activityRate/${params}`, {}, 'GET');
// 统计动态发布
export const getNewsRate = params => ajax(`${backendAPI}/api/message/newsRate/${params}`, {}, 'GET');
// 收支增长率
export const getAccountGrowthRate = params => ajax(`${backendAPI}/api/financial/accountGrowthRate/${params}`, {}, 'GET');
// 收入比重
export const getIncomePercentage = params => ajax(`${backendAPI}/api/financial/incomePercentage/${params}`, {}, 'GET');
// 统计指定月份中各摘要的排名
export const getOrderByAmount = params => ajax(`${backendAPI}/api/financial/orderByAmount/${params}`, {}, 'GET');
// 统计指定指定日期月份前6个月的账单
export const getPreSixMonthBill = params => ajax(`${backendAPI}/api/financial/preSixMonthBill/${params}`, {}, 'GET');
// 查询单条便笺
export const getMemo = params => ajax(`${backendAPI}/api/message/memo/show`, params, 'GET');
// 获取分页便笺
export const getMemoList = params => ajax(`${backendAPI}/api/message/memo`, params, 'GET');
// 添加便笺
export const createMemo = params => ajax(`${backendAPI}/api/message/memo/create`, params, 'POST');
// 修改便笺
export const updateMemo = params => ajax(`${backendAPI}/api/message/memo/edit`, params, 'PUT');
// 删除便笺
export const deleteMemo = params => ajax(`${backendAPI}/api/message/memo/delete`, params, 'DELETE');
