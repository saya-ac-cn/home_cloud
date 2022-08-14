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
// {{host}}:{{port}}/backend/oss/files/page

// 删除文件
export const deleteFile = params => ajax(`${backendAPI}/api/oss/files/delete`, params, 'DELETE');
// {{host}}:{{port}}/backend/oss/files/27

// 修改文件
export const editFile = params => ajax(`${backendAPI}/api/oss/files/edit`, params, 'PUT');
// {{host}}:{{port}}/backend/oss/files/file

// 下载文件
export const downloadFileForAdmin = `${backendAPI}/api/oss/files/download/`;
// {{host}}:{{port}}/backend/oss/files/download/29

// 创建笔记簿
export const createNoteBook = params => ajax(`${backendAPI}/api/message/notebook/create`, params, 'POST');
// {{host}}:{{port}}/backend/content/notebook

// 修改笔记簿
export const updateNoteBook = params => ajax(`${backendAPI}/api/message/notebook/edit`, params, 'PUT');
// {{host}}:{{port}}/backend/content/notebook

// 删除笔记簿
export const deleteNoteBook = params => ajax(`${backendAPI}/api/message/notebook/delete`, params, 'DELETE');
// {{host}}:{{port}}/backend/content/notebook/14

// 获取笔记簿列表
export const getNoteBookGroup = params => ajax(`${backendAPI}/api/message/notebook/group`, params, 'GET');
// {{host}}:{{port}}/backend/content/notebook

// 创建笔记
export const createNotes = params => ajax(`${backendAPI}/api/message/notes/create`, params, 'POST');
// {{host}}:{{port}}/backend/content/notes

// 修改笔记
export const updateNotes = params => ajax(`${backendAPI}/api/message/notes/edit`, params, 'PUT');
// {{host}}:{{port}}/backend/content/notes

// 删除笔记
export const deleteNotes = params => ajax(`${backendAPI}/api/message/notes/delete`, params, 'DELETE');
// {{host}}:{{port}}/backend/content/notes/15

// 笔记分页
export const getNotesList = params => ajax(`${backendAPI}/api/message/notes`, params, 'GET');
// {{host}}:{{port}}/backend/content/notes

// 查询单条笔记
export const getNotes = params => ajax(`${backendAPI}/api/message/notes/show`, params, 'GET');
// {{host}}:{{port}}/backend/content/notes/15

// 获取该月计划
export const getPlanList = params => ajax(`${backendAPI}/api/set/plan`, params, 'GET');
// 添加计划
export const createPlan = params => ajax(`${backendAPI}/api/set/plan/create`, params, 'POST');
// 修改计划
export const updatePlan = params => ajax(`${backendAPI}/api/set/plan/edit`, params, 'PUT');
// 删除计划
export const deletePlan = params => ajax(`${backendAPI}/api/set/plan/delete`, params, 'DELETE');

// 查询货币列表
// {{host}}:{{port}}/backend/financial/dictionary/monetary

// 获取所有的支付类别
export const getFinancialType = params => ajax(`${backendAPI}/api/financial/transactionType`, params, 'GET');
// {{host}}:{{port}}/backend/financial/dictionary/payment/means

// 获取所有的交易摘要
export const getFinancialAmount = params => ajax(`${backendAPI}/api/financial/transactionAmount`, params, 'GET');
// {{host}}:{{port}}/backend/financial/dictionary/abstracts

// 获取财政流水
export const getTransactionList = params => ajax(`${backendAPI}/api/financial/transaction`, params, 'GET');
// {{host}}:{{port}}/backend/financial/journal

// 查看收支明细（明细记录折叠存）
export const getTransactionDetail = params => ajax(`${backendAPI}/api/financial/transactionDetail`, params, 'GET');

// 申报流水
export const applyTransaction = params => ajax(`${backendAPI}/api/financial/insertTransaction`, params, 'POST');
// {{host}}:{{port}}/backend/financial/journal

// 修改流水
export const updateTransaction = params => ajax(`${backendAPI}/api/financial/updateTransaction`, params, 'PUT');
// {{host}}:{{port}}/backend/financial/journal

// 删除流水
export const deleteTransaction = params => ajax(`${backendAPI}/api/financial/deleteTransaction`, params, 'DELETE');
// {{host}}:{{port}}/backend/financial/journal/825

// 导出流水
export const downTransaction = `${backendAPI}/api/financial/outTransactionListExcel`;
// {{host}}:{{port}}/backend/financial/journal/excel

// 导出流水明细
export const outTransactionInfoExcel = `${backendAPI}/api/financial/outTransactionInfoExcel`;
// {{host}}:{{port}}/backend/financial/general/journal/excel

// 添加流水明细
export const insertTransactioninfo = params => ajax(`${backendAPI}/api/financial/insertTransactioninfo`, params, 'POST');
// {{host}}:{{port}}/backend/financial/general/journal

// 修改流水明细
export const updateTransactioninfo = params => ajax(`${backendAPI}/api/financial/updateTransactioninfo`, params, 'PUT');
// {{host}}:{{port}}/backend/financial/general/journal

// 删除流水明细
export const deleteTransactioninfo = params => ajax(`${backendAPI}/api/financial/deleteTransactioninfo`, params, 'DELETE');
// {{host}}:{{port}}/backend/financial/general/journal/11

// 按天汇总统计流水
export const totalTransactionForDay = params => ajax(`${backendAPI}/api/financial/totalTransactionForDay`, params, 'GET');
// {{host}}:{{port}}/backend/financial/journal/day

// 导出按天汇总的报表
export const outTransactionForDayExcel = `${backendAPI}/api/financial/outTransactionForDayExcel`;
// {{host}}:{{port}}/backend/financial/journal/collect/excel


// 查看数据库备份执行列表
export const getBackUpDBList = params => ajax(`${backendAPI}/api/oss/db`, params, 'GET');
// 下载备份的数据库文件
export const downloadBackUpDB = `${backendAPI}/api/oss/db/download/`;

// 获取数据总量及词云数据
export const getCountAndWordCloud = () => ajax(`${backendAPI}/api/set/countAndWordCloud`, {}, 'GET');

// 查询活跃度
export const getActivityRate= params => ajax(`${backendAPI}/api/set/activityRate/${params}`, {}, 'GET');
// {{host}}:{{port}}/backend/system/log/total/pre6

// 统计动态发布
export const getNewsRate = params => ajax(`${backendAPI}/api/message/newsRate/${params}`, {}, 'GET');

// 收支增长率
export const getAccountGrowthRate = params => ajax(`${backendAPI}/api/financial/accountGrowthRate/${params}`, {}, 'GET');
// {{host}}:{{port}}/backend/financial/journal/total/balance

// 收入比重
export const getIncomePercentage = params => ajax(`${backendAPI}/api/financial/incomePercentage/${params}`, {}, 'GET');
// {{host}}:{{port}}/backend/financial/journal/total/income

// 统计指定月份中各摘要的排名
export const getOrderByAmount = params => ajax(`${backendAPI}/api/financial/orderByAmount/${params}`, {}, 'GET');
// {{host}}:{{port}}/backend/financial/journal/total/order

// 统计指定指定日期月份前6个月的账单
export const getPreSixMonthBill = params => ajax(`${backendAPI}/api/financial/preSixMonthBill/${params}`, {}, 'GET');
// {{host}}:{{port}}/backend/financial/journal/total/pre6

// 查询单条便笺
export const getMemo = params => ajax(`${backendAPI}/api/message/memo/show`, params, 'GET');
// {{host}}:{{port}}/backend/content/memo/6

// 获取分页便笺
export const getMemoList = params => ajax(`${backendAPI}/api/message/memo`, params, 'GET');
// {{host}}:{{port}}/backend/content/memo

// 添加便笺
export const createMemo = params => ajax(`${backendAPI}/api/message/memo/create`, params, 'POST');
// {{host}}:{{port}}/backend/content/memo

// 修改便笺
export const updateMemo = params => ajax(`${backendAPI}/api/message/memo/edit`, params, 'PUT');
// {{host}}:{{port}}/backend/content/memo

// 删除便笺
export const deleteMemo = params => ajax(`${backendAPI}/api/message/memo/delete`, params, 'DELETE');
// {{host}}:{{port}}/backend/content/memo/6