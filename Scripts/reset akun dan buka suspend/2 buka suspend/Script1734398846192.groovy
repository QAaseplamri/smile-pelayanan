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

WebUI.setText(findTestObject('Object Repository/reset akun dan buka suspend/2 buka suspend/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/input__login'), 
    'EK262740')

WebUI.setEncryptedText(findTestObject('Object Repository/reset akun dan buka suspend/2 buka suspend/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/input__password'), 
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

WebUI.click(findTestObject('Object Repository/reset akun dan buka suspend/2 buka suspend/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/div_CSO - Customer Service Officer ( J0P )'))

WebUI.click(findTestObject('Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_Pilih_button-1025-btnIconEl'))

WebUI.doubleClick(findTestObject('Object Repository/reset akun dan buka suspend/1 reset akun/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_Pelayanan'))

WebUI.doubleClick(findTestObject('Object Repository/reset akun dan buka suspend/1 reset akun/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_Informasi dan Pengaduan'))

WebUI.scrollToElement(findTestObject('reset akun dan buka suspend/1 reset akun/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_LD5001-Perekaman Informasi dan Pengaduan'), 
    0)

WebUI.click(findTestObject('Object Repository/reset akun dan buka suspend/1 reset akun/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_LD5001-Perekaman Informasi dan Pengaduan'))

WebUI.click(findTestObject('Object Repository/reset akun dan buka suspend/2 buka suspend/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/a_New'))

WebUI.click(findTestObject('Object Repository/reset akun dan buka suspend/2 buka suspend/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/img'))

WebUI.switchToWindowTitle('TOPIK INTERAKSI')

WebUI.click(findTestObject('Object Repository/reset akun dan buka suspend/2 buka suspend/Page_TOPIK INTERAKSI/a_BUKA BLOKIR PENGKINIAN DATA JMO'))

WebUI.switchToWindowTitle('SMILE - Sistem Informasi Perlindungan Pekerja (46)')

WebUI.setText(findTestObject('Object Repository/reset akun dan buka suspend/2 buka suspend/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/input__tb_nomor_identitas'), 
    '3671123103000003')

WebUI.takeScreenshot(RunConfiguration.getProjectDir() + '/Screenshoot/reset akun dan buka suspend/buka-suspend-data-salahInput-sukses.png')

WebUI.setText(findTestObject('Object Repository/reset akun dan buka suspend/2 buka suspend/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/textarea_(max 4000 karakter)_tb_keterangan'), 
    'TINDAKLANJUT SALAH INPUT KARTU PADA PENGKINIAN DATA JMO UNTUK NOMOR IDENTITAS 3671123103000003 DAN EMAIL LANGILANG642@GMAIL.COM')

WebUI.delay(5)

WebUI.click(findTestObject('Object Repository/reset akun dan buka suspend/2 buka suspend/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/fieldset_Entry Antrian Klaim               _e10b52'))

WebUI.click(findTestObject('Object Repository/reset akun dan buka suspend/2 buka suspend/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/input_Status Buka Suspend_flag_disclaimer_s_56fd81'))

WebUI.click(findTestObject('Object Repository/reset akun dan buka suspend/2 buka suspend/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/input__btn_submit'))

WebUI.click(findTestObject('Object Repository/reset akun dan buka suspend/2 buka suspend/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_Yes_button-1006-btnIconEl'))

WebUI.delay(10)

WebUI.takeScreenshot(RunConfiguration.getProjectDir() + '/Screenshoot/reset akun dan buka suspend/buka-suspend-submit-salahInput-sukses.png')

WebUI.click(findTestObject('Object Repository/reset akun dan buka suspend/2 buka suspend/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/input__butentry'))

WebUI.click(findTestObject('Object Repository/reset akun dan buka suspend/2 buka suspend/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/img'))

WebUI.switchToWindowTitle('TOPIK INTERAKSI')

WebUI.click(findTestObject('Object Repository/reset akun dan buka suspend/2 buka suspend/Page_TOPIK INTERAKSI/a_GAGAL PROSES BIOMETRIK'))

WebUI.switchToWindowTitle('SMILE - Sistem Informasi Perlindungan Pekerja (46)')

WebUI.setText(findTestObject('Object Repository/reset akun dan buka suspend/2 buka suspend/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/input__tb_nomor_identitas'), 
    '1273041702930001')

WebUI.takeScreenshot(RunConfiguration.getProjectDir() + '/Screenshoot/reset akun dan buka suspend/buka-suspend-data-gagalBiometrik-sukses.png')

WebUI.setText(findTestObject('Object Repository/reset akun dan buka suspend/2 buka suspend/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/textarea_(max 4000 karakter)_tb_keterangan'), 
    'TINDAKLANJUT GAGAL BIOMETRIK PENGKINIAN DATA JMO UNTUK NOMOR IDENTITAS 1273041702930001 DAN EMAIL ALI.RAHMANHARAHAP17@GMAIL.COM')

WebUI.delay(5)

WebUI.click(findTestObject('Object Repository/reset akun dan buka suspend/2 buka suspend/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/fieldset_Entry Antrian Klaim               _e10b52'))

WebUI.click(findTestObject('Object Repository/reset akun dan buka suspend/2 buka suspend/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/input_Status Buka Suspend_flag_disclaimer_g_8d5eee'))

WebUI.click(findTestObject('Object Repository/reset akun dan buka suspend/2 buka suspend/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/input__btn_submit'))

WebUI.click(findTestObject('Object Repository/reset akun dan buka suspend/2 buka suspend/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_Yes_button-1006-btnIconEl'))

WebUI.delay(5)

WebUI.takeScreenshot(RunConfiguration.getProjectDir() + '/Screenshoot/reset akun dan buka suspend/buka-suspend-submit-gagalBiometrik-sukses.png')

