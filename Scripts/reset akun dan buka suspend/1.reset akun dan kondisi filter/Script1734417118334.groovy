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
import com.kms.katalon.core.configuration.RunConfiguration as RunConfiguration

WebUI.openBrowser('')

WebUI.navigateToUrl('http://172.28.108.46:5337/smile/login.bpjs')

WebUI.setText(findTestObject('Object Repository/reset akun dan buka suspend/1 reset akun/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/input__login'), 
    'EK262740')

WebUI.setEncryptedText(findTestObject('Object Repository/reset akun dan buka suspend/1 reset akun/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/input__password'), 
    'gKpkrQPjjYE=')

boolean isVisible = WebUI.verifyElementVisible(findTestObject('Blasting Campaign/Page_SMILE - Sistem Informasi Perlindungan Pekerja (46)/buton_Yes'), 
    FailureHandling.OPTIONAL)

if (isVisible) {
    WebUI.click(findTestObject('Object Repository/New Folder/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_Yes_button-1006-btnIconEl'))

    WebUI.click(findTestObject('Object Repository/New Folder/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_OK_button-1005-btnIconEl'))

    WebUI.click(findTestObject('Object Repository/New Folder/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_Login_button-1016-btnIconEl'))
} else {
    WebUI.comment('Button Yes login tidak ditemukan!')

    WebUI.click(findTestObject('Blasting Campaign/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_Login_button-1016-btnIconEl'))
}

WebUI.doubleClick(findTestObject('Object Repository/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/input__rule'))

WebUI.setText(findTestObject('Object Repository/reset akun dan buka suspend/1 reset akun/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/input__rule'), 
    'J0P')

WebUI.click(findTestObject('Object Repository/reset akun dan buka suspend/1 reset akun/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/div_ADMP - Administrator ( J0P )'))

WebUI.click(findTestObject('Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_Pilih_button-1025-btnIconEl'))

WebUI.doubleClick(findTestObject('Object Repository/reset akun dan buka suspend/1 reset akun/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_Pelayanan'))

WebUI.doubleClick(findTestObject('Object Repository/reset akun dan buka suspend/1 reset akun/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_Informasi dan Pengaduan'))

WebUI.scrollToElement(findTestObject('reset akun dan buka suspend/1 reset akun/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_LD5001-Perekaman Informasi dan Pengaduan'), 
    0)

WebUI.click(findTestObject('Object Repository/reset akun dan buka suspend/1 reset akun/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_LD5001-Perekaman Informasi dan Pengaduan'))

WebUI.click(findTestObject('Object Repository/reset akun dan buka suspend/1 reset akun/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/a_New'))

WebUI.click(findTestObject('Object Repository/reset akun dan buka suspend/1 reset akun/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/img'))

WebUI.switchToWindowTitle('TOPIK INTERAKSI')

WebUI.click(findTestObject('Object Repository/reset akun dan buka suspend/1 reset akun/Page_TOPIK INTERAKSI/a_HAPUS AKUN JMO'))

WebUI.switchToWindowTitle('SMILE - Sistem Informasi Perlindungan Pekerja (46)')

WebUI.click(findTestObject('Object Repository/reset akun dan buka suspend/1 reset akun/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/img_1'))

WebUI.switchToWindowTitle('TOPIK PESERTA')

WebUI.setText(findTestObject('Object Repository/reset akun dan buka suspend/1 reset akun/Page_TOPIK PESERTA/input_Search By_searchtxt'), 
    '3216060801680014')

WebUI.click(findTestObject('Object Repository/reset akun dan buka suspend/1 reset akun/Page_TOPIK PESERTA/input_Search By_cari2'))

WebUI.takeScreenshot(RunConfiguration.getProjectDir() + '/Screenshoot/reset akun dan buka suspend/nik-3data-sukses.png')

WebUI.delay(5)

WebUI.selectOptionByValue(findTestObject('Object Repository/reset akun dan buka suspend/1 reset akun/Page_TOPIK PESERTA/select_No Identitas            Email       _a1eaf6'), 
    'sc_email', true)

WebUI.setText(findTestObject('Object Repository/reset akun dan buka suspend/1 reset akun/Page_TOPIK PESERTA/input_Search By_searchtxt'), 
    'alautumn22@gmail.com')

WebUI.click(findTestObject('Object Repository/reset akun dan buka suspend/1 reset akun/Page_TOPIK PESERTA/input_Search By_cari2'))

WebUI.takeScreenshot(RunConfiguration.getProjectDir() + '/Screenshoot/reset akun dan buka suspend/email-1data-sukses.png')

WebUI.delay(5)

WebUI.selectOptionByValue(findTestObject('Object Repository/reset akun dan buka suspend/1 reset akun/Page_TOPIK PESERTA/select_No Identitas            Email       _a1eaf6'), 
    'sc_handphone', true)

WebUI.setText(findTestObject('Object Repository/reset akun dan buka suspend/1 reset akun/Page_TOPIK PESERTA/input_Search By_searchtxt'), 
    '083899570311')

WebUI.click(findTestObject('Object Repository/reset akun dan buka suspend/1 reset akun/Page_TOPIK PESERTA/input_Search By_cari2'))

WebUI.takeScreenshot(RunConfiguration.getProjectDir() + '/Screenshoot/reset akun dan buka suspend/HP-10data-sukses.png')

WebUI.delay(5)

WebUI.selectOptionByValue(findTestObject('Object Repository/reset akun dan buka suspend/1 reset akun/Page_TOPIK PESERTA/select_No Identitas            Email       _a1eaf6'), 
    'sc_email', true)

WebUI.setText(findTestObject('Object Repository/reset akun dan buka suspend/1 reset akun/Page_TOPIK PESERTA/input_Search By_searchtxt'), 
    'alfianrd14@gmail.com')

WebUI.click(findTestObject('Object Repository/reset akun dan buka suspend/1 reset akun/Page_TOPIK PESERTA/input_Search By_cari2'))

WebUI.takeScreenshot(RunConfiguration.getProjectDir() + '/Screenshoot/reset akun dan buka suspend/email-upper-lower-sukses.png')

WebUI.delay(5)

WebUI.selectOptionByValue(findTestObject('Object Repository/reset akun dan buka suspend/1 reset akun/Page_TOPIK PESERTA/select_No Identitas            Email       _a1eaf6'), 
    'sc_handphone', true)

WebUI.setText(findTestObject('Object Repository/reset akun dan buka suspend/1 reset akun/Page_TOPIK PESERTA/input_Search By_searchtxt'), 
    '081546805801')

WebUI.click(findTestObject('Object Repository/reset akun dan buka suspend/1 reset akun/Page_TOPIK PESERTA/input_Search By_cari2'))

WebUI.delay(5)

WebUI.doubleClick(findTestObject('Object Repository/reset akun dan buka suspend/1 reset akun/Page_TOPIK PESERTA/input_Search By_searchtxt'))

WebUI.setText(findTestObject('Object Repository/reset akun dan buka suspend/1 reset akun/Page_TOPIK PESERTA/input_Search By_searchtxt'), 
    '62081546805801')

WebUI.click(findTestObject('Object Repository/reset akun dan buka suspend/1 reset akun/Page_TOPIK PESERTA/input_Search By_cari2'))

WebUI.delay(5)

WebUI.doubleClick(findTestObject('Object Repository/reset akun dan buka suspend/1 reset akun/Page_TOPIK PESERTA/input_Search By_searchtxt'))

WebUI.setText(findTestObject('Object Repository/reset akun dan buka suspend/1 reset akun/Page_TOPIK PESERTA/input_Search By_searchtxt'), 
    '6281546805801')

WebUI.click(findTestObject('Object Repository/reset akun dan buka suspend/1 reset akun/Page_TOPIK PESERTA/input_Search By_cari2'))

WebUI.delay(5)

WebUI.takeScreenshot(RunConfiguration.getProjectDir() + '/Screenshoot/reset akun dan buka suspend/HP-628-6208-08-sukses.png')

WebUI.selectOptionByValue(findTestObject('Object Repository/reset akun dan buka suspend/1 reset akun/Page_TOPIK PESERTA/select_No Identitas            Email       _a1eaf6'), 
    'sc_no_identitas', true)

WebUI.setText(findTestObject('Object Repository/reset akun dan buka suspend/1 reset akun/Page_TOPIK PESERTA/input_Search By_searchtxt'), 
    '1374024704900001')

WebUI.click(findTestObject('Object Repository/reset akun dan buka suspend/1 reset akun/Page_TOPIK PESERTA/input_Search By_cari2'))

WebUI.click(findTestObject('Object Repository/reset akun dan buka suspend/1 reset akun/Page_TOPIK PESERTA/a_LUZI RAMADHANI'))

WebUI.switchToWindowTitle('SMILE - Sistem Informasi Perlindungan Pekerja (46)')

WebUI.delay(15)

WebUI.click(findTestObject('Object Repository/reset akun dan buka suspend/1 reset akun/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/img_1'))

WebUI.switchToWindowTitle('TOPIK PESERTA')

WebUI.setText(findTestObject('Object Repository/reset akun dan buka suspend/1 reset akun/Page_TOPIK PESERTA/input_Search By_searchtxt'), 
    '3273302610910001')

WebUI.click(findTestObject('Object Repository/reset akun dan buka suspend/1 reset akun/Page_TOPIK PESERTA/input_Search By_cari2'))

WebUI.click(findTestObject('Object Repository/reset akun dan buka suspend/1 reset akun/Page_TOPIK PESERTA/a_WANDI WITARSA'))

WebUI.switchToWindowTitle('SMILE - Sistem Informasi Perlindungan Pekerja (46)')

WebUI.selectOptionByValue(findTestObject('Object Repository/reset akun dan buka suspend/1 reset akun/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/select_-- Pilih Status Pengajuan --Kantor P_9e48f2'), 
    '0', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/reset akun dan buka suspend/1 reset akun/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/select_-- Pilih Status Pengajuan --Kantor P_9e48f2'), 
    'Kantor Cabang', true)

WebUI.setText(findTestObject('Object Repository/reset akun dan buka suspend/1 reset akun/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/textarea_(max 4000 karakter)_tb_keterangan'), 
    'UJI COBA KATALON')

WebUI.click(findTestObject('Object Repository/reset akun dan buka suspend/1 reset akun/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/input_Nama Ibu Kandung_flag_disclaimer_reset'))

WebUI.click(findTestObject('Object Repository/reset akun dan buka suspend/1 reset akun/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/input__btn_submit'))

WebUI.click(findTestObject('Object Repository/reset akun dan buka suspend/1 reset akun/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_Yes_button-1006-btnIconEl'))

WebUI.delay(5)

WebUI.takeScreenshot(RunConfiguration.getProjectDir() + '/Screenshoot/reset akun dan buka suspend/reset akun-sukses.png')

WebUI.closeBrowser()

