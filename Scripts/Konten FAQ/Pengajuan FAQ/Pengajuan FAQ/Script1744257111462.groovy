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
import java.util.regex.Matcher as Matcher
import java.util.regex.Pattern as Pattern

WebUI.openBrowser('')

WebUI.maximizeWindow()

WebUI.navigateToUrl(GlobalVariable.url)

WebUI.click(findTestObject('Object Repository/Konten faq/Pengajuan FAQ/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/input__login'), 
    FailureHandling.STOP_ON_FAILURE)

WebUI.setText(findTestObject('Object Repository/Konten faq/Pengajuan FAQ/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/input__login'), 
    GlobalVariable.username)

WebUI.click(findTestObject('Object Repository/Konten faq/Pengajuan FAQ/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/input__password'), 
    FailureHandling.STOP_ON_FAILURE)

WebUI.setText(findTestObject('Object Repository/Konten faq/Pengajuan FAQ/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/input__password'), 
    GlobalVariable.password)

WebUI.click(findTestObject('Object Repository/Konten faq/Pengajuan FAQ/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_Login_button-1016-btnIconEl'))

boolean isVisible = WebUI.verifyElementVisible(findTestObject('Object Repository/Konten faq/Pengajuan FAQ/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_Yes_button-1006-btnIconEl'), 
    FailureHandling.OPTIONAL)

if (isVisible) {
    WebUI.click(findTestObject('Object Repository/Konten faq/Pengajuan FAQ/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_Yes_button-1006-btnIconEl'))

    WebUI.click(findTestObject('Object Repository/Konten faq/Pengajuan FAQ/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_OK_button-1005-btnIconEl'))

    WebUI.click(findTestObject('Object Repository/Konten faq/Pengajuan FAQ/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_Login_button-1016-btnIconEl'))
} else {
    WebUI.comment('Button Yes login tidak ditemukan!')
}

//WebUI.click(findTestObject('Object Repository/Konten faq/Pengajuan FAQ/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_Yes_button-1006-btnIconEl'))
//
//WebUI.click(findTestObject('Object Repository/Konten faq/Pengajuan FAQ/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_OK_button-1005-btnIconEl'))
//
//WebUI.click(findTestObject('Object Repository/Konten faq/Pengajuan FAQ/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_Login_button-1016-btnIconEl'))
WebUI.click(findTestObject('Filtering Data/Edit CMS/Page_SMILE - Sistem Informasi Perlindungan Pekerja (46)/input_role'))

WebUI.clearText(findTestObject('Filtering Data/Edit CMS/Page_SMILE - Sistem Informasi Perlindungan Pekerja (46)/input_role'))

'Input Role'
WebUI.setText(findTestObject('Filtering Data/Edit CMS/Page_SMILE - Sistem Informasi Perlindungan Pekerja (46)/input_role'), 
    'ppmf - petugas pengelola cms faq ( 0 )')

WebUI.sendKeys(findTestObject('Filtering Data/Edit CMS/Page_SMILE - Sistem Informasi Perlindungan Pekerja (46)/input_role'), 
    Keys.chord(Keys.ENTER))

WebUI.click(findTestObject('Object Repository/Filtering Data/Filtering Data/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_Pilih_button-1025-btnIconEl'))

WebUI.doubleClick(findTestObject('Object Repository/Konten faq/Pengajuan FAQ/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_e-Channel'))

WebUI.doubleClick(findTestObject('Object Repository/Konten faq/Pengajuan FAQ/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_JMO'))

WebUI.click(findTestObject('Object Repository/Konten faq/Pengajuan FAQ/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_AS1032-CMS FAQ'))

WebUI.click(findTestObject('Object Repository/Konten faq/Pengajuan FAQ/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/a_New'))

WebUI.waitForElementVisible(findTestObject('Object Repository/Konten faq/Pengajuan FAQ/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/a_New'), 
    60)

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

// Ambil teks dari pop-up
String popupText = WebUI.getText(findTestObject('Object Repository/Konten faq/Pengajuan FAQ/Data ke 1/Page_SMILE - Sistem Informasi Perlindungan Pekerja (46)/alert kode pengajuan'))

println('Teks dari pop-up: ' + popupText)

WebUI.delay(2)

WebUI.takeScreenshot()

WebUI.click(findTestObject('Object Repository/Konten faq/Pengajuan FAQ/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_OK_button-1005-btnIconEl'))

WebUI.click(findTestObject('Object Repository/Konten faq/Pengajuan FAQ/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/img'))

WebUI.click(findTestObject('Filtering Data/Edit CMS/Page_SMILE - Sistem Informasi Perlindungan Pekerja (46)/input_role'))

WebUI.clearText(findTestObject('Filtering Data/Edit CMS/Page_SMILE - Sistem Informasi Perlindungan Pekerja (46)/input_role'))

'Input Role'
WebUI.setText(findTestObject('Filtering Data/Edit CMS/Page_SMILE - Sistem Informasi Perlindungan Pekerja (46)/input_role'), 
    'ppe - penata pelayanan elektronik ( 0 )')

WebUI.sendKeys(findTestObject('Filtering Data/Edit CMS/Page_SMILE - Sistem Informasi Perlindungan Pekerja (46)/input_role'), 
    Keys.chord(Keys.ENTER))

WebUI.click(findTestObject('Object Repository/Filtering Data/Filtering Data/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_Pilih_button-1025-btnIconEl'))

WebUI.doubleClick(findTestObject('Konten faq/Pengajuan FAQ/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_e-Channel'))

WebUI.doubleClick(findTestObject('Object Repository/Konten faq/Pengajuan FAQ/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_JMO'))

WebUI.scrollToElement(findTestObject('Konten faq/Pengajuan FAQ/Page_SMILE - Sistem Informasi Perlindungan Pekerja (46)/span_AS1034- Approval Pengajuan Penyesuaian CMS FAQ'), 
    0)

WebUI.doubleClick(findTestObject('Konten faq/Pengajuan FAQ/Page_SMILE - Sistem Informasi Perlindungan Pekerja (46)/span_AS1034- Approval Pengajuan Penyesuaian CMS FAQ'))

WebUI.waitForElementVisible(findTestObject('Object Repository/Konten faq/Pengajuan FAQ/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/input_x_search_by'), 
    60)

WebUI.click(findTestObject('Object Repository/Konten faq/Pengajuan FAQ/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/input_x_search_by'), 
    FailureHandling.STOP_ON_FAILURE)

// Gunakan regex untuk menemukan kode pengajuan (format FAQxxxxxxxxxx)
Pattern pattern = Pattern.compile('`(FAQ\\d+)`')

Matcher matcher = pattern.matcher(popupText)

String kodePengajuan = ''

if (matcher.find()) {
    kodePengajuan = matcher.group(1 // Ambil hanya kode pengajuannya
        )

    println('Kode Pengajuan: ' + kodePengajuan)
}

WebUI.delay(2)

WebUI.setText(findTestObject('Object Repository/Konten faq/Pengajuan FAQ/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/input_x_search_by'), 
    kodePengajuan)

// Klik tombol cari
WebUI.click(findTestObject('Object Repository/Konten faq/Pengajuan FAQ/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/input_x_btncari'))

WebUI.delay(2)

WebUI.takeScreenshot()

//
// Ambil teks hasil pencarian untuk verifikasi
//String hasilCari = WebUI.getText(findTestObject('Object Repository/Konten faq/Pengajuan FAQ/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/a_Tindak Lanjut'))
//
//println('Hasil pencarian: ' + hasilCari)
//WebUI.setText(findTestObject('Object Repository/Konten faq/Pengajuan FAQ/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/input_x_search_by'), 
//    'FAQ250203033')
//
//WebUI.click(findTestObject('Object Repository/Konten faq/Pengajuan FAQ/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/input_x_btncari'))
WebUI.click(findTestObject('Object Repository/Konten faq/Pengajuan FAQ/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/a_Tindak Lanjut'))

WebUI.click(findTestObject('Object Repository/Konten faq/Pengajuan FAQ/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/input__status_approval'))

WebUI.setText(findTestObject('Object Repository/Konten faq/Pengajuan FAQ/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/textarea__keterangan_approval'), 
    'YA PPE')

WebUI.click(findTestObject('Object Repository/Konten faq/Pengajuan FAQ/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/button_Submit'))

WebUI.click(findTestObject('Object Repository/Konten faq/Pengajuan FAQ/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_Yes_button-1006-btnIconEl'))

WebUI.delay(2)

WebUI.takeScreenshot()

WebUI.click(findTestObject('Object Repository/Konten faq/Pengajuan FAQ/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_OK_button-1005-btnIconEl'))

WebUI.click(findTestObject('Object Repository/Konten faq/Pengajuan FAQ/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/img'))

WebUI.click(findTestObject('Filtering Data/Edit CMS/Page_SMILE - Sistem Informasi Perlindungan Pekerja (46)/input_role'))

WebUI.clearText(findTestObject('Filtering Data/Edit CMS/Page_SMILE - Sistem Informasi Perlindungan Pekerja (46)/input_role'))

'Input Role'
WebUI.setText(findTestObject('Filtering Data/Edit CMS/Page_SMILE - Sistem Informasi Perlindungan Pekerja (46)/input_role'), 
    'asdpe - asdep bidang pelayanan elektronik ( 0 )')

WebUI.sendKeys(findTestObject('Filtering Data/Edit CMS/Page_SMILE - Sistem Informasi Perlindungan Pekerja (46)/input_role'), 
    Keys.chord(Keys.ENTER))

WebUI.click(findTestObject('Object Repository/Filtering Data/Filtering Data/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_Pilih_button-1025-btnIconEl'))

WebUI.doubleClick(findTestObject('Object Repository/Konten faq/Pengajuan FAQ/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_e-Channel'))

WebUI.doubleClick(findTestObject('Object Repository/Konten faq/Pengajuan FAQ/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_JMO'))

WebUI.doubleClick(findTestObject('Object Repository/Konten faq/Pengajuan FAQ/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_AS1034- Approval Pengajuan Penyesuaian_eef70b'))

WebUI.waitForElementVisible(findTestObject('Object Repository/Konten faq/Pengajuan FAQ/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/input_x_search_by'), 
    60)

// Input kode pengajuan ke dalam kolom pencarian
WebUI.click(findTestObject('Object Repository/Konten faq/Pengajuan FAQ/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/input_x_search_by'), 
    FailureHandling.STOP_ON_FAILURE)

//WebUI.setText(findTestObject('Object Repository/Konten faq/Pengajuan FAQ/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/input_x_search_by'), 
//    'FAQ250203033')
// Gunakan regex untuk menemukan kode pengajuan (format FAQxxxxxxxxxx)
//Pattern pattern = Pattern.compile('`(FAQ\\d+)`')
//
//Matcher matcher = pattern.matcher(popupText)
//String kodePengajuan = ''
if (matcher.find()) {
    kodePengajuan = matcher.group(1 // Ambil hanya kode pengajuannya
        )

    println('Kode Pengajuan: ' + kodePengajuan)
}

WebUI.delay(2)

WebUI.setText(findTestObject('Object Repository/Konten faq/Pengajuan FAQ/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/input_x_search_by'), 
    kodePengajuan)

// Klik tombol cari
WebUI.click(findTestObject('Object Repository/Konten faq/Pengajuan FAQ/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/input_x_btncari'))

WebUI.delay(2)

WebUI.takeScreenshot()

WebUI.click(findTestObject('Object Repository/Konten faq/Pengajuan FAQ/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/a_Tindak Lanjut'))

WebUI.click(findTestObject('Object Repository/Konten faq/Pengajuan FAQ/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/input__status_approval'))

WebUI.setText(findTestObject('Object Repository/Konten faq/Pengajuan FAQ/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/textarea__keterangan_approval'), 
    'YA ASDPE')

WebUI.click(findTestObject('Object Repository/Konten faq/Pengajuan FAQ/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/button_Submit'))

WebUI.click(findTestObject('Object Repository/Konten faq/Pengajuan FAQ/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_Yes_button-1006-btnIconEl'))

WebUI.delay(2)

WebUI.takeScreenshot()

WebUI.click(findTestObject('Object Repository/Konten faq/Pengajuan FAQ/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_OK_button-1005-btnIconEl'))

WebUI.click(findTestObject('Object Repository/Konten faq/Pengajuan FAQ/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/img'))

WebUI.click(findTestObject('Filtering Data/Edit CMS/Page_SMILE - Sistem Informasi Perlindungan Pekerja (46)/input_role'))

WebUI.clearText(findTestObject('Filtering Data/Edit CMS/Page_SMILE - Sistem Informasi Perlindungan Pekerja (46)/input_role'))

'Input Role'
WebUI.setText(findTestObject('Filtering Data/Edit CMS/Page_SMILE - Sistem Informasi Perlindungan Pekerja (46)/input_role'), 
    'deppep - depdir bidang pep ( 24a )')

WebUI.sendKeys(findTestObject('Filtering Data/Edit CMS/Page_SMILE - Sistem Informasi Perlindungan Pekerja (46)/input_role'), 
    Keys.chord(Keys.ENTER))

WebUI.click(findTestObject('Object Repository/Filtering Data/Filtering Data/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_Pilih_button-1025-btnIconEl'))

WebUI.doubleClick(findTestObject('Object Repository/Konten faq/Pengajuan FAQ/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_e-Channel'))

WebUI.doubleClick(findTestObject('Object Repository/Konten faq/Pengajuan FAQ/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_JMO'))

WebUI.doubleClick(findTestObject('Object Repository/Konten faq/Pengajuan FAQ/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_AS1034- Approval Pengajuan Penyesuaian_eef70b'))

WebUI.waitForElementVisible(findTestObject('Object Repository/Konten faq/Pengajuan FAQ/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/input_x_search_by'), 
    60)

// Input kode pengajuan ke dalam kolom pencarian
WebUI.click(findTestObject('Object Repository/Konten faq/Pengajuan FAQ/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/input_x_search_by'), 
    FailureHandling.STOP_ON_FAILURE)

// Gunakan regex untuk menemukan kode pengajuan (format FAQxxxxxxxxxx)
//Pattern pattern = Pattern.compile('`(FAQ\\d+)`')
//
//Matcher matcher = pattern.matcher(popupText)
//String kodePengajuan = ''
if (matcher.find()) {
    kodePengajuan = matcher.group(1 // Ambil hanya kode pengajuannya
        )

    println('Kode Pengajuan: ' + kodePengajuan)
}

WebUI.delay(2)

WebUI.setText(findTestObject('Object Repository/Konten faq/Pengajuan FAQ/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/input_x_search_by'), 
    kodePengajuan)

// Klik tombol cari
WebUI.click(findTestObject('Object Repository/Konten faq/Pengajuan FAQ/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/input_x_btncari'))

//WebUI.setText(findTestObject('Object Repository/Konten faq/Pengajuan FAQ/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/input_x_search_by'), 
//    'FAQ250203033')
//
//WebUI.click(findTestObject('Object Repository/Konten faq/Pengajuan FAQ/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/input_x_btncari'))
WebUI.click(findTestObject('Object Repository/Konten faq/Pengajuan FAQ/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/a_Tindak Lanjut'))

WebUI.click(findTestObject('Object Repository/Konten faq/Pengajuan FAQ/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/input__status_approval'))

WebUI.setText(findTestObject('Object Repository/Konten faq/Pengajuan FAQ/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/textarea__keterangan_approval'), 
    'YA DEFPEP')

WebUI.click(findTestObject('Object Repository/Konten faq/Pengajuan FAQ/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/button_Submit'))

WebUI.click(findTestObject('Object Repository/Konten faq/Pengajuan FAQ/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_Yes_button-1006-btnIconEl'))

WebUI.delay(2)

WebUI.takeScreenshot()

WebUI.click(findTestObject('Object Repository/Konten faq/Pengajuan FAQ/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_OK_button-1005-btnIconEl'))

WebUI.click(findTestObject('Object Repository/Konten faq/Pengajuan FAQ/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/img'))

WebUI.click(findTestObject('Filtering Data/Edit CMS/Page_SMILE - Sistem Informasi Perlindungan Pekerja (46)/input_role'))

WebUI.clearText(findTestObject('Filtering Data/Edit CMS/Page_SMILE - Sistem Informasi Perlindungan Pekerja (46)/input_role'))

'Input Role'
WebUI.setText(findTestObject('Filtering Data/Edit CMS/Page_SMILE - Sistem Informasi Perlindungan Pekerja (46)/input_role'), 
    'ppmf - petugas pengelola cms faq ( 0 )')

WebUI.sendKeys(findTestObject('Filtering Data/Edit CMS/Page_SMILE - Sistem Informasi Perlindungan Pekerja (46)/input_role'), 
    Keys.chord(Keys.ENTER))

WebUI.click(findTestObject('Object Repository/Filtering Data/Filtering Data/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_Pilih_button-1025-btnIconEl'))

WebUI.doubleClick(findTestObject('Object Repository/Konten faq/Pengajuan FAQ/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_e-Channel'))

WebUI.doubleClick(findTestObject('Object Repository/Konten faq/Pengajuan FAQ/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_JMO'))

WebUI.doubleClick(findTestObject('Konten faq/Pengajuan FAQ/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/AS1033-CMS FAQ Monitoring Approval'))

WebUI.waitForElementVisible(findTestObject('Konten faq/Pengajuan FAQ/Data ke 1/Page_SMILE - Sistem Informasi Perlindungan Pekerja (46)/input_Kode Pengajuan_search_txt4'), 
    60)

// Input kode pengajuan ke dalam kolom pencarian
WebUI.click(findTestObject('Konten faq/Pengajuan FAQ/Data ke 1/Page_SMILE - Sistem Informasi Perlindungan Pekerja (46)/input_Kode Pengajuan_search_txt4'), 
    FailureHandling.STOP_ON_FAILURE)

if (matcher.find()) {
    kodePengajuan = matcher.group(1 // Ambil hanya kode pengajuannya
        )

    println('Kode Pengajuan: ' + kodePengajuan)
}

WebUI.delay(2)

WebUI.setText(findTestObject('Konten faq/Pengajuan FAQ/Data ke 1/Page_SMILE - Sistem Informasi Perlindungan Pekerja (46)/input_Kode Pengajuan_search_txt4'), 
    kodePengajuan)

// Klik tombol cari
WebUI.click(findTestObject('Konten faq/Pengajuan FAQ/Data ke 1/Page_SMILE - Sistem Informasi Perlindungan Pekerja (46)/input_Kode Pengajuan_btnfilter'))

WebUI.delay(2)

WebUI.takeScreenshot()

