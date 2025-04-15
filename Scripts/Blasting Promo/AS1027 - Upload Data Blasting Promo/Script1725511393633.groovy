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
import java.awt.Robot as Robot
import java.awt.event.KeyEvent as KeyEvent

WebUI.openBrowser('')

WebUI.maximizeWindow()

WebUI.navigateToUrl(GlobalVariable.Url_5332)

WebUI.setText(findTestObject('Object Repository/Blasting Promo/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/input__login'), 
    GlobalVariable.username1)

WebUI.setText(findTestObject('Object Repository/Blasting Promo/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/input__password'), 
    GlobalVariable.password)

WebUI.click(findTestObject('Object Repository/Blasting Promo/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_Login_button-1016-btnIconEl'))

WebUI.click(findTestObject('Filtering Data/Edit CMS/Page_SMILE - Sistem Informasi Perlindungan Pekerja (46)/input_role'))

WebUI.clearText(findTestObject('Filtering Data/Edit CMS/Page_SMILE - Sistem Informasi Perlindungan Pekerja (46)/input_role'))

'Input Role'
WebUI.setText(findTestObject('Filtering Data/Edit CMS/Page_SMILE - Sistem Informasi Perlindungan Pekerja (46)/input_role'), 
    'PPE - Penata Pelayanan Elektronik ( 0 )')

WebUI.sendKeys(findTestObject('Filtering Data/Edit CMS/Page_SMILE - Sistem Informasi Perlindungan Pekerja (46)/input_role'), 
    Keys.chord(Keys.ENTER))

WebUI.click(findTestObject('Object Repository/Blasting Promo/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_Pilih_button-1025-btnIconEl'))

WebUI.doubleClick(findTestObject('Object Repository/Blasting Promo/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_e-Channel'))

WebUI.doubleClick(findTestObject('Object Repository/Blasting Promo/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_JMO'))

WebUI.click(findTestObject('Object Repository/Blasting Promo/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_AS1027-Upload Data Blasting Promo'))

WebUI.click(findTestObject('Object Repository/Blasting Promo/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/a_New'))

WebUI.uploadFile(findTestObject('Blasting Promo/Page_SMILE - Sistem Informasi Perlindungan Pekerja (46)/input__fileUpload'), 
    'C:\\Users\\Admin\\Downloads\\deeplink_promo.csv')

WebUI.setText(findTestObject('Object Repository/Blasting Promo/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/textarea_Keterangan_keterangan'), 
    'Promo')

WebUI.takeScreenshot()

WebUI.click(findTestObject('Object Repository/Blasting Promo/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/input_Keterangan_btn_simpan'))

WebUI.takeScreenshot()

WebUI.click(findTestObject('Object Repository/Blasting Promo/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_Yes_button-1006-btnIconEl'))

WebUI.takeScreenshot()

WebUI.click(findTestObject('Object Repository/Blasting Promo/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_OK_button-1005-btnIconEl'))

WebUI.delay(2)

WebUI.takeScreenshot()

WebUI.click(findTestObject('Object Repository/Blasting Promo/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/a_View'))

WebUI.delay(2)

WebUI.takeScreenshot()

WebUI.click(findTestObject('Object Repository/Blasting Promo/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/input_Aksi_btn_Download'))

WebUI.delay(2)

WebUI.takeScreenshot()

WebUI.switchToWindowTitle('SMILE - Sistem Informasi Perlindungan Pekerja (46)')

WebUI.click(findTestObject('Object Repository/Blasting Promo/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/a_View_1'))

WebUI.takeScreenshot()

WebUI.click(findTestObject('Object Repository/Blasting Promo/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/input_Aksi_btn_Download'))

WebUI.delay(2)

WebUI.takeScreenshot()

WebUI.click(findTestObject('Object Repository/Blasting Promo/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_OK_button-1005-btnIconEl'))

WebUI.delay(2)

WebUI.takeScreenshot()

