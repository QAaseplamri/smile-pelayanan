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

WebUI.setText(findTestObject('Object Repository/Filtering Data/Filtering Data/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/input__login'), 
    GlobalVariable.username)

WebUI.setText(findTestObject('Object Repository/Filtering Data/Filtering Data/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/input__password'), 
    GlobalVariable.password)

WebUI.click(findTestObject('Object Repository/Report Blasting Campaign/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_Login_button-1016-btnIconEl'))

WebUI.click(findTestObject('Filtering Data/Edit CMS/Page_SMILE - Sistem Informasi Perlindungan Pekerja (46)/input_role'))

WebUI.clearText(findTestObject('Filtering Data/Edit CMS/Page_SMILE - Sistem Informasi Perlindungan Pekerja (46)/input_role'))

WebUI.setText(findTestObject('Filtering Data/Edit CMS/Page_SMILE - Sistem Informasi Perlindungan Pekerja (46)/input_role'), 
    'PPE - Penata Pelayanan Elektronik ( 0 )')

WebUI.sendKeys(findTestObject('Filtering Data/Edit CMS/Page_SMILE - Sistem Informasi Perlindungan Pekerja (46)/input_role'), 
    Keys.chord(Keys.ENTER))

WebUI.click(findTestObject('Object Repository/Report Blasting Campaign/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_Pilih_button-1025-btnIconEl'))

WebUI.doubleClick(findTestObject('Object Repository/Report Blasting Campaign/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_e-Channel'))

WebUI.doubleClick(findTestObject('Object Repository/Report Blasting Campaign/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/div_JMO'))

WebUI.click(findTestObject('Object Repository/Report Blasting Campaign/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_AS1025-Report BlastingCampaign'))

WebUI.click(findTestObject('Object Repository/Report Blasting Campaign/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_-- Pilih Status Kriteria --'))

WebUI.setText(findTestObject('Object Repository/Report Blasting Campaign/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/search__field'), 
    'testing data1')

WebUI.sendKeys(findTestObject('Report Blasting Campaign/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/search__field'), 
    Keys.chord(Keys.ENTER))

WebUI.click(findTestObject('Object Repository/Report Blasting Campaign/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_-- Pilih Tahun --'))

WebUI.setText(findTestObject('Object Repository/Report Blasting Campaign/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/search__field'), 
    '2024')

WebUI.sendKeys(findTestObject('Report Blasting Campaign/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/search__field'), 
    Keys.chord(Keys.ENTER))

WebUI.click(findTestObject('Object Repository/Report Blasting Campaign/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_Tampilkan'))

WebUI.takeScreenshot()

WebUI.click(findTestObject('Report Blasting Campaign/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/context_menu'), 
    FailureHandling.STOP_ON_FAILURE)

WebUI.click(findTestObject('Object Repository/Report Blasting Campaign/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/li_Download JPEG image'), 
    FailureHandling.STOP_ON_FAILURE)

WebUI.click(findTestObject('Report Blasting Campaign/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/context_menu'))

WebUI.click(findTestObject('Object Repository/Report Blasting Campaign/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/li_Download PDF document'))

WebUI.click(findTestObject('Report Blasting Campaign/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/context_menu'))

WebUI.click(findTestObject('Object Repository/Report Blasting Campaign/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/li_Download CSV'))

