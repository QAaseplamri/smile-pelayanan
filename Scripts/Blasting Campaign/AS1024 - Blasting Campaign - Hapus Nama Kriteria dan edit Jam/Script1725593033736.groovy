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

WebUI.openBrowser('')

WebUI.maximizeWindow()

WebUI.navigateToUrl(GlobalVariable.Url_5331)

WebUI.setText(findTestObject('Object Repository/Blasting Campaign/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/input__login'), 
    GlobalVariable.username)

WebUI.setText(findTestObject('Object Repository/Blasting Campaign/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/input__password'), 
    GlobalVariable.password)

WebUI.click(findTestObject('Object Repository/Blasting Campaign/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_Login_button-1016-btnIconEl'))

WebUI.click(findTestObject('Object Repository/Blasting Campaign/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/div__ext-gen1096'))

WebUI.click(findTestObject('Filtering Data/Edit CMS/Page_SMILE - Sistem Informasi Perlindungan Pekerja (46)/input_role'))

WebUI.clearText(findTestObject('Filtering Data/Edit CMS/Page_SMILE - Sistem Informasi Perlindungan Pekerja (46)/input_role'))

WebUI.setText(findTestObject('Filtering Data/Edit CMS/Page_SMILE - Sistem Informasi Perlindungan Pekerja (46)/input_role'), 
    'PPE - Penata Pelayanan Elektronik ( 0 )')

WebUI.sendKeys(findTestObject('Filtering Data/Edit CMS/Page_SMILE - Sistem Informasi Perlindungan Pekerja (46)/input_role'), 
    Keys.chord(Keys.ENTER))

WebUI.click(findTestObject('Object Repository/Blasting Campaign/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_Pilih_button-1025-btnIconEl'))

WebUI.doubleClick(findTestObject('Object Repository/Blasting Campaign/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_e-Channel'))

WebUI.doubleClick(findTestObject('Object Repository/Blasting Campaign/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_JMO'))

WebUI.click(findTestObject('Object Repository/Blasting Campaign/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_AS1024-BlastingCampaign'))

WebUI.delay(2)

WebUI.click(findTestObject('Object Repository/Blasting Campaign/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/a_New'))

WebUI.click(findTestObject('Object Repository/Blasting Campaign/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/li_Notifications'))

WebUI.setText(findTestObject('Object Repository/Blasting Campaign/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/input_Notification'), 
    'Hari Natal')

WebUI.setText(findTestObject('Object Repository/Blasting Campaign/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/textarea_Notification'), 
    'Natal')

WebUI.click(findTestObject('Object Repository/Blasting Campaign/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/button_Next'))

WebUI.click(findTestObject('Blasting Campaign/Page_SMILE - Sistem Informasi Perlindungan Pekerja (46)/span_-- Pilih kriteria --'))

WebUI.click(findTestObject('Blasting Campaign/Page_SMILE - Sistem Informasi Perlindungan Pekerja (46)/earch__field_kriteria'))

WebUI.setText(findTestObject('Blasting Campaign/Page_SMILE - Sistem Informasi Perlindungan Pekerja (46)/earch__field_kriteria'), 
    'satu')

WebUI.sendKeys(findTestObject('Blasting Campaign/Page_SMILE - Sistem Informasi Perlindungan Pekerja (46)/earch__field_kriteria'), 
    Keys.chord(Keys.ENTER))

WebUI.click(findTestObject('Blasting Campaign/Page_SMILE - Sistem Informasi Perlindungan Pekerja (46)/button_and'))

WebUI.click(findTestObject('Blasting Campaign/Page_SMILE - Sistem Informasi Perlindungan Pekerja (46)/earch__field_kriteria_add'))

WebUI.setText(findTestObject('Blasting Campaign/Page_SMILE - Sistem Informasi Perlindungan Pekerja (46)/earch__field_kriteria_add_seach'), 
    'dua')

WebUI.sendKeys(findTestObject('Blasting Campaign/Page_SMILE - Sistem Informasi Perlindungan Pekerja (46)/earch__field_kriteria_add_seach'), 
    Keys.chord(Keys.ENTER))

WebUI.takeScreenshot()

WebUI.click(findTestObject('Object Repository/Blasting Campaign/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/button_Next_1'))

WebUI.click(findTestObject('Blasting Campaign/Page_SMILE - Sistem Informasi Perlindungan Pekerja (46)/button_Sekarang'))

WebUI.click(findTestObject('Blasting Campaign/Page_SMILE - Sistem Informasi Perlindungan Pekerja (46)/div_Terjadwal'))

WebUI.click(findTestObject('Blasting Campaign/Page_SMILE - Sistem Informasi Perlindungan Pekerja (46)/input_Sekarang'))

WebUI.setText(findTestObject('Blasting Campaign/Page_SMILE - Sistem Informasi Perlindungan Pekerja (46)/input_Sekarang'), 
    '09/12/2024')

WebUI.sendKeys(findTestObject('Blasting Campaign/Page_SMILE - Sistem Informasi Perlindungan Pekerja (46)/input_Sekarang'), 
    Keys.chord(Keys.ENTER))

WebUI.takeScreenshot()

WebUI.click(findTestObject('Blasting Campaign/Page_SMILE - Sistem Informasi Perlindungan Pekerja (46)/button_prev_1'))

WebUI.click(findTestObject('Blasting Campaign/Page_SMILE - Sistem Informasi Perlindungan Pekerja (46)/button_hapus'))

WebUI.click(findTestObject('Blasting Campaign/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/button_Next_1'))

WebUI.click(findTestObject('Blasting Campaign/Page_SMILE - Sistem Informasi Perlindungan Pekerja (46)/div_jam'))

WebUI.setText(findTestObject('Blasting Campaign/Page_SMILE - Sistem Informasi Perlindungan Pekerja (46)/div_jam'), '23:00')

WebUI.sendKeys(findTestObject('Blasting Campaign/Page_SMILE - Sistem Informasi Perlindungan Pekerja (46)/div_jam'), Keys.chord(
        Keys.ENTER))

WebUI.click(findTestObject('Blasting Campaign/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/button_Submit'))

WebUI.takeScreenshot()

WebUI.click(findTestObject('Blasting Campaign/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_Yes_button-1006-btnIconEl'))

WebUI.takeScreenshot()

WebUI.click(findTestObject('Blasting Campaign/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_OK_button-1005-btnIconEl'))

WebUI.delay(1)

WebUI.takeScreenshot()

WebUI.selectOptionByValue(findTestObject('Object Repository/Blasting Campaign/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/select_Pilih_Nama BlastingCampaign'), 
    'NAMA_KRITERIA', true)

WebUI.setText(findTestObject('Object Repository/Blasting Campaign/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/input_Search by_keyword'), 
    'Hari Natal')

WebUI.click(findTestObject('Object Repository/Blasting Campaign/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/input_Search by_btncari'))

WebUI.delay(2)

WebUI.click(findTestObject('Blasting Campaign/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/edit'))

WebUI.click(findTestObject('Blasting Campaign/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/button_Next'))

WebUI.click(findTestObject('Blasting Campaign/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/button_Next_1'))

WebUI.click(findTestObject('Blasting Campaign/Test/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/kalender_edit'))

WebUI.click(findTestObject('Blasting Campaign/Page_SMILE - Sistem Informasi Perlindungan Pekerja (46)/div_Terjadwal_edit'))

WebUI.setText(findTestObject('Blasting Campaign/Test/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/input_tanggal_edit'), 
    '09/11/2024')

WebUI.setText(findTestObject('Blasting Campaign/Page_SMILE - Sistem Informasi Perlindungan Pekerja (46)/Jam_edit'), '14:00')

WebUI.click(findTestObject('Blasting Campaign/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/button_Submit'))

WebUI.takeScreenshot()

WebUI.click(findTestObject('Blasting Campaign/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_Yes_button-1006-btnIconEl'))

WebUI.takeScreenshot()

WebUI.click(findTestObject('Blasting Campaign/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_OK_button-1005-btnIconEl'))

WebUI.takeScreenshot()

