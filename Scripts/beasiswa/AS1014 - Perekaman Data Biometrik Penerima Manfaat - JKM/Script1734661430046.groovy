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
import redisKeywords.RedisUtils as RedisUtils

WebUI.openBrowser('')

WebUI.navigateToUrl('http://172.28.108.46:5441/smile/login.bpjs')

WebUI.setText(findTestObject('Object Repository/beasiswa/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/input__login'), 
    'EK262740')

WebUI.setEncryptedText(findTestObject('Object Repository/beasiswa/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/input__password'), 
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

WebUI.setText(findTestObject('Object Repository/beasiswa/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/input__rule'), 
    'J00')

WebUI.click(findTestObject('Object Repository/beasiswa/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/div_CSO - Customer Service Officer ( J0P )'))

WebUI.click(findTestObject('Object Repository/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_Pilih_button-1025-btnIconEl'))

WebUI.doubleClick(findTestObject('Object Repository/beasiswa/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_e-Channel'))

WebUI.doubleClick(findTestObject('Object Repository/beasiswa/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_JMO'))

WebUI.scrollToElement(findTestObject('beasiswa/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_AS1014-Perekaman Data Biometrik Pelapor'), 
    0)

WebUI.click(findTestObject('Object Repository/beasiswa/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_AS1014-Perekaman Data Biometrik Pelapor'))

WebUI.click(findTestObject('Object Repository/beasiswa/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_Entry'))

not_run: WebUI.click(findTestObject('Object Repository/beasiswa/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/input_Manfaat Beasiswa JKK_tipe_manfaat'))

WebUI.click(findTestObject('beasiswa/manual/radio button jkm/Page_SMILE - Sistem Informasi Perlindungan Pekerja (46)/input_Manfaat Beasiswa JKM_tipe_manfaat'))

WebUI.click(findTestObject('Object Repository/beasiswa/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/img'))

WebUI.switchToWindowTitle('SIJSTK')

WebUI.setText(findTestObject('Object Repository/beasiswa/Page_SIJSTK/input_Search By_searchtxt'), '3171054404830003')

WebUI.click(findTestObject('Object Repository/beasiswa/Page_SIJSTK/input_Search By_cari2'))

WebUI.click(findTestObject('Object Repository/beasiswa/Page_SIJSTK/td_AGUNG PRIHATONO'))

WebUI.switchToWindowTitle('SMILE - Sistem Informasi Perlindungan Pekerja (46)')

not_run: WebUI.selectOptionByValue(findTestObject('Object Repository/beasiswa/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/select_-- Pilih Jenis Penerima Manfaat --  _efcf89'), 
    'Y', true)

WebUI.click(findTestObject('beasiswa/manual/radiobutton/Page_SMILE - Sistem Informasi Perlindungan Pekerja (46)/input_Pelapor Awal_jenis_pelapor'))

WebUI.selectOptionByValue(findTestObject('Object Repository/beasiswa/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/select_-- Pilih Status Hubungan--PENERIMA B_6d0f48'), 
    'B2', true)

WebUI.setText(findTestObject('Object Repository/beasiswa/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/input_Ibu Kandung_ibu_kandung_pelapor'), 
    'SALENA')

WebUI.selectOptionByValue(findTestObject('Object Repository/beasiswa/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/select_-- Pilih Golongan Darah--AABBO'), 
    'A', true)

WebUI.click(findTestObject('beasiswa/manual/Page_SMILE - Sistem Informasi Perlindungan Pekerja (46)/img'))

WebUI.switchToWindowTitle('SIJSTK')

WebUI.click(findTestObject('Object Repository/beasiswa/Page_SIJSTK/a_Gambir'))

WebUI.switchToWindowTitle('SMILE - Sistem Informasi Perlindungan Pekerja (46)')

WebUI.setText(findTestObject('Object Repository/beasiswa/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/input_NISN (No Induk Siswa Nasional)_nisn_pelapor'), 
    '0028490630')

WebUI.click(findTestObject('Object Repository/beasiswa/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/input_NISN (No Induk Siswa Nasional)_btn_val_nisn'))

WebUI.click(findTestObject('Object Repository/beasiswa/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/input_NISN Tidak Ditemukan_btn_simpan_pelapor'))

WebUI.click(findTestObject('Object Repository/beasiswa/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_Yes_button-1006-btnIconEl'))

WebUI.click(findTestObject('Object Repository/beasiswa/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_OK_button-1005-btnIconEl'))

WebUI.takeScreenshot(RunConfiguration.getProjectDir() + '/Screenshoot/beasiswa/AS1014-sukses_stepone.png')

WebUI.setText(findTestObject('beasiswa/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/input_Email_email_pelapor'), 'KATALON_BEASISWA001@GMAIL.COM')

WebUI.click(findTestObject('beasiswa/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/input_Email_btn_aktifasi_email'))

WebUI.click(findTestObject('Object Repository/beasiswa/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_Yes_button-1006-btnIconEl'))

not_run: WebUI.click(findTestObject('Object Repository/beasiswa/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_OK_button-1005-btnIconEl'))

WebUI.delay(20)

not_run: WebUI.setText(findTestObject('beasiswa/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/input_Email_otp_email'), 
    '589683')

//List<Map> redisServers = [[('host') : '172.28.108.231', ('port') : 7000, ('password') : 'redisclusterDEV#' // Server Redis pertama
//    ], [('host') : '172.28.108.235', ('port') : 7000, ('password') : 'redisclusterDEV#' // Server Redis kedua
//    ], [('host') : '172.28.108.231', ('port') : 7001, ('password') : 'redisclusterDEV#' // Server Redis ketiga
//    ]]
//
//// Key yang digunakan untuk mendapatkan OTP
//String smsotpkey = 'SMILE#AS1014#otp#0813815728211'
//
//// Mendapatkan OTP dari beberapa Redis server
//String otpsms = RedisUtils.getOTPFromMultipleRedisSMS(redisServers, smsotpkey)
//
//if (otpsms != null) {
//    println('OTP yang didapat dari Redis: ' + otpsms)
//
//    // Lanjutkan untuk mengisi OTP di aplikasi
//    //	WebUI.setText(findTestObject('Page_Login/input_otp'), otpCode)
//    //	Mobile.setText(findTestObject('Object Repository/Aktivasi Akun/PU/android.widget.EditText (1)'), otpsms, 2)
//    WebUI.setText(findTestObject('Object Repository/beasiswa/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/input_No Handphone_otp_hp'), 
//        otpsms, 2)
//} else {
//    println('Gagal mendapatkan OTP dari semua Redis servers.')
//}
WebUI.click(findTestObject('beasiswa/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/input_Email_btn_otp_email'))

WebUI.click(findTestObject('Object Repository/beasiswa/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_Yes_button-1006-btnIconEl'))

WebUI.click(findTestObject('Object Repository/beasiswa/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_OK_button-1005-btnIconEl'))

WebUI.setText(findTestObject('Object Repository/beasiswa/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/input_No Handphone_handphone_pelapor'), 
    '0813815717616')

WebUI.click(findTestObject('Object Repository/beasiswa/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/input_No Handphone_btn_aktifasi_hp'))

WebUI.click(findTestObject('Object Repository/beasiswa/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_Yes_button-1006-btnIconEl'))

not_run: WebUI.click(findTestObject('beasiswa/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_OK_button-1005-btnIconEl'))

WebUI.delay(20)

not_run: WebUI.setText(findTestObject('Object Repository/beasiswa/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/input_No Handphone_otp_hp'), 
    '589683')

//List<Map> redisServers = [[('host') : '172.28.108.231', ('port') : 7000, ('password') : 'redisclusterDEV#' // Server Redis pertama
//    ], [('host') : '172.28.108.235', ('port') : 7000, ('password') : 'redisclusterDEV#' // Server Redis kedua
//    ], [('host') : '172.28.108.231', ('port') : 7001, ('password') : 'redisclusterDEV#' // Server Redis ketiga
//    ]]
//
//// Key yang digunakan untuk mendapatkan OTP
//String smsotpkey = 'SMILE#AS1014#otp#0813815728211'
//
//// Mendapatkan OTP dari beberapa Redis server
//String otpsms = RedisUtils.getOTPFromMultipleRedisSMS(redisServers, smsotpkey)
//
//if (otpsms != null) {
//    println('OTP yang didapat dari Redis: ' + otpsms)
//
//    // Lanjutkan untuk mengisi OTP di aplikasi
//    //	WebUI.setText(findTestObject('Page_Login/input_otp'), otpCode)
//    //	Mobile.setText(findTestObject('Object Repository/Aktivasi Akun/PU/android.widget.EditText (1)'), otpsms, 2)
//    WebUI.setText(findTestObject('Object Repository/beasiswa/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/input_No Handphone_otp_hp'), 
//        otpsms, 2)
//} else {
//    println('Gagal mendapatkan OTP dari semua Redis servers.')
//}
WebUI.click(findTestObject('Object Repository/beasiswa/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/input_No Handphone_btn_otp_hp'))

WebUI.click(findTestObject('Object Repository/beasiswa/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_Yes_button-1006-btnIconEl'))

WebUI.click(findTestObject('Object Repository/beasiswa/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_OK_button-1005-btnIconEl'))

WebUI.click(findTestObject('Object Repository/beasiswa/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/input_Foto Penerima Manfaat_btn green'))

WebUI.switchToWindowTitle('Demo - Capture Photo From Webcam Using Javascript')

not_run: WebUI.click(findTestObject('Object Repository/beasiswa/Page_Demo - Capture Photo From Webcam Using_006fea/button_Capture Photo'))

WebUI.delay(15)

WebUI.switchToWindowTitle('SMILE - Sistem Informasi Perlindungan Pekerja (46)')

WebUI.click(findTestObject('Object Repository/beasiswa/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_OK_button-1005-btnIconEl'))

WebUI.setText(findTestObject('Object Repository/beasiswa/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/textarea_Keterangan hasil verifikasi_ketera_896fd6'), 
    'KATALON_BEASISWA001@GMAIL.COM')

WebUI.click(findTestObject('Object Repository/beasiswa/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/input_Alasan Penolakan_flag_disclaimer'))

not_run: WebUI.delay(60)

WebUI.click(findTestObject('Object Repository/beasiswa/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/input_Alasan Penolakan_btn-simpan-2'))

WebUI.click(findTestObject('Object Repository/beasiswa/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_Yes_button-1006-btnIconEl'))

WebUI.takeScreenshot(RunConfiguration.getProjectDir() + '/Screenshoot/beasiswa/AS1014-sukses_steptwo.png')

not_run: WebUI.click(findTestObject('Object Repository/beasiswa/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_OK_button-1005-btnIconEl'))

