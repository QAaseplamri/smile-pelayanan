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

WebUI.navigateToUrl('http://172.28.108.46:5441/smile/login.bpjs')

WebUI.setText(findTestObject('Object Repository/New Folder/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/input__login'), 
    'EK262740')

WebUI.setEncryptedText(findTestObject('Object Repository/New Folder/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/input__password'), 
    'gKpkrQPjjYE=')

WebUI.click(findTestObject('Object Repository/New Folder/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_Login_button-1016-btnIconEl'))

WebUI.click(findTestObject('Object Repository/New Folder/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_Yes_button-1006-btnIconEl'))

WebUI.click(findTestObject('Object Repository/New Folder/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_OK_button-1005-btnIconEl'))

WebUI.doubleClick(findTestObject('Object Repository/New Folder/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/input__rule'))

WebUI.setText(findTestObject('Object Repository/New Folder/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/input__rule'), 
    'J0P')

WebUI.click(findTestObject('Object Repository/New Folder/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/div_CSO - Customer Service Officer ( J0P )'))

WebUI.doubleClick(findTestObject('Object Repository/New Folder/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_e-Channel'))

WebUI.click(findTestObject('Object Repository/New Folder/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_JMO'))

WebUI.doubleClick(findTestObject('Object Repository/New Folder/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_JMO'))

WebUI.click(findTestObject('Object Repository/New Folder/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_AS1014-Perekaman Data Biometrik Pelapor'))

WebUI.closeBrowser()

