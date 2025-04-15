import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

WebUI.callTestCase(findTestCase('01-Login/Login-LoginToSMILE-01_Success'), [:], FailureHandling.STOP_ON_FAILURE)

String inisial = findTestData('Data Files/Pengajuan FAQ/data_role').getValue('inisial', 1)

String role = findTestData('Data Files/Pengajuan FAQ/data_role').getValue('role', 1)

WebUI.callTestCase(findTestCase('01-Login/Login-pilihRoleSSMILE-02_Success'), [('inisial') : inisial, ('role') : role], 
    FailureHandling.STOP_ON_FAILURE)

CustomKeywords.'sectionMenu.utilityMenu.selectMenu'('e-Channel')

CustomKeywords.'sectionMenu.utilityMenu.selectMenu'('JMO')

CustomKeywords.'sectionMenu.utilityMenu.selectMenu'('AS1032-CMS FAQ')

not_run: WebUI.doubleClick(findTestObject('Object Repository/Konten faq/Pengajuan FAQ/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_e-Channel'))

not_run: WebUI.doubleClick(findTestObject('Object Repository/Konten faq/Pengajuan FAQ/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_JMO'))

not_run: WebUI.click(findTestObject('Object Repository/Konten faq/Pengajuan FAQ/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_AS1032-CMS FAQ'))

WebUI.waitForElementVisible(findTestObject('Object Repository/Konten faq/Pengajuan FAQ/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/a_New'), 
    0)

WebUI.click(findTestObject('Object Repository/Konten faq/Pengajuan FAQ/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/a_New'))

WebUI.click(findTestObject('Object Repository/Konten faq/Pengajuan FAQ/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_-- Pilih Kategori --'))

WebUI.delay(2)

WebUI.setText(findTestObject('Konten faq/Pengajuan FAQ/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_-- Pilih Kategori --_1'), 
    'FKAT001 – UMUM')

WebUI.sendKeys(findTestObject('Konten faq/Pengajuan FAQ/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_-- Pilih Kategori --_1'), 
    Keys.chord(Keys.ENTER), FailureHandling.STOP_ON_FAILURE)

WebUI.click(findTestObject('Object Repository/Konten faq/Pengajuan FAQ/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_-- Pilih Sub Kategori --_1'))

WebUI.delay(2)

WebUI.setText(findTestObject('Konten faq/Pengajuan FAQ/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_-- Pilih Sub Kategori --_2'), 
    'FSUB0001 - PENGERTIAN UMUM', FailureHandling.STOP_ON_FAILURE)

WebUI.sendKeys(findTestObject('Konten faq/Pengajuan FAQ/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_-- Pilih Sub Kategori --_2'), 
    Keys.chord(Keys.ENTER), FailureHandling.STOP_ON_FAILURE)

WebUI.click(findTestObject('Konten faq/Pengajuan FAQ/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/data-1'))

WebUI.setText(findTestObject('Konten faq/Pengajuan FAQ/Data ke 1/Page_SMILE - Sistem Informasi Perlindungan Pekerja (46)/pertanyaan'), 
    'Ceritakan tentang diri kamu')

WebUI.setText(findTestObject('Konten faq/Pengajuan FAQ/Data ke 1/Page_SMILE - Sistem Informasi Perlindungan Pekerja (46)/jawaban'), 
    'Nama saya Indira lulusan Universitas Bandung S1 Ilmu Komunikasi. Saat ini saya ingin mencoba mengisi posisi yang dibutuhkan, yaitu di bidang humas. Selama masa kuliah, saya aktif dalam organisasi kampus dan bergabung dengan komunitas di sana.\n\nDengan kemampuan yang saya miliki, maka saya yakin bisa berkontribusi langsung dengan perusahaan.\nNama saya Indira lulusan Universitas Bandung S1 Ilmu Komunikasi. Saat ini saya ingin mencoba mengisi posisi yang dibutuhkan, yaitu di bidang humas. Selama masa kuliah, saya aktif dalam organisasi kampus dan bergabung dengan komunitas di sana.\n\nDengan kemampuan yang saya miliki, maka saya yakin bisa berkontribusi langsung dengan perusahaan.\n')

WebUI.click(findTestObject('Object Repository/Konten faq/Pengajuan FAQ/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/textarea__keterangan'), 
    FailureHandling.STOP_ON_FAILURE)

WebUI.setText(findTestObject('Object Repository/Konten faq/Pengajuan FAQ/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/textarea__keterangan'), 
    'Perkenalan saat interview di Perusahaan')

WebUI.click(findTestObject('Object Repository/Konten faq/Pengajuan FAQ/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/input__btn_simpan'))

WebUI.click(findTestObject('Object Repository/Konten faq/Pengajuan FAQ/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_Yes_button-1006-btnIconEl'))

// Ambil teks lengkap dari alert pop-up
String teksPopUp = WebUI.getText(findTestObject('Object Repository/Konten faq/Pengajuan FAQ/Data ke 1/Page_SMILE - Sistem Informasi Perlindungan Pekerja (46)/alert kode pengajuan'))

// Ekstrak hanya kode pengajuannya pakai regex
def matcher = teksPopUp =~ '(FAQ\\d{9})'

if (matcher.find()) {
    String kodePengajuan = matcher.group(1)

    GlobalVariable.kodePengajuan = kodePengajuan

    println('Kode Pengajuan berhasil diambil: ' + kodePengajuan)
} else {
    WebUI.comment('Gagal mengambil kode pengajuan dari teks pop-up: ' + teksPopUp)
}

WebUI.delay(2)

WebUI.takeScreenshot()

WebUI.click(findTestObject('Object Repository/Konten faq/Pengajuan FAQ/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_OK_button-1005-btnIconEl'))

