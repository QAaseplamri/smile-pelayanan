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

WebUI.navigateToUrl(GlobalVariable.Url_5332)

WebUI.setText(findTestObject('Object Repository/Perekaman Data Biometrik Penerima Manfaat/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/input__login'), 
    GlobalVariable.username)

WebUI.setText(findTestObject('Object Repository/Perekaman Data Biometrik Penerima Manfaat/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/input__password'), 
    GlobalVariable.password)

WebUI.click(findTestObject('Object Repository/Perekaman Data Biometrik Penerima Manfaat/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_Login_button-1016-btnIconEl'))

WebUI.click(findTestObject('Filtering Data/Edit CMS/Page_SMILE - Sistem Informasi Perlindungan Pekerja (46)/input_role'))

WebUI.clearText(findTestObject('Filtering Data/Edit CMS/Page_SMILE - Sistem Informasi Perlindungan Pekerja (46)/input_role'))

'Input Role'
WebUI.setText(findTestObject('Filtering Data/Edit CMS/Page_SMILE - Sistem Informasi Perlindungan Pekerja (46)/input_role'), 
    'cso - customer service officer ( j0p )')

WebUI.sendKeys(findTestObject('Filtering Data/Edit CMS/Page_SMILE - Sistem Informasi Perlindungan Pekerja (46)/input_role'), 
    Keys.chord(Keys.ENTER))

WebUI.click(findTestObject('Object Repository/Perekaman Data Biometrik Penerima Manfaat/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_Pilih_button-1025-btnIconEl'))

WebUI.doubleClick(findTestObject('Object Repository/Perekaman Data Biometrik Penerima Manfaat/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_e-Channel'))

WebUI.doubleClick(findTestObject('Object Repository/Perekaman Data Biometrik Penerima Manfaat/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_JMO'))

WebUI.click(findTestObject('Object Repository/Perekaman Data Biometrik Penerima Manfaat/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_AS1014-Perekaman Data Biometrik Pelapor'))

WebUI.delay(2)

WebUI.click(findTestObject('Object Repository/Perekaman Data Biometrik Penerima Manfaat/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_Entry'))

WebUI.delay(2)

WebUI.click(findTestObject('Object Repository/Perekaman Data Biometrik Penerima Manfaat/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/img'))

WebUI.switchToWindowTitle('SIJSTK')

WebUI.setText(findTestObject('Object Repository/Perekaman Data Biometrik Penerima Manfaat/Page_SIJSTK/input_Search By_searchtxt'), 
    NIK)

WebUI.click(findTestObject('Object Repository/Perekaman Data Biometrik Penerima Manfaat/Page_SIJSTK/input_Search By_cari2'))

WebUI.delay(2)

WebUI.takeScreenshot()

WebUI.click(findTestObject('Object Repository/Perekaman Data Biometrik Penerima Manfaat/Page_SIJSTK/td_NIK'))

WebUI.switchToWindowUrl(GlobalVariable.Url_5332)

WebUI.verifyElementVisible(findTestObject('Perekaman Data Biometrik Penerima Manfaat/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/img'))

WebUI.takeScreenshot()

