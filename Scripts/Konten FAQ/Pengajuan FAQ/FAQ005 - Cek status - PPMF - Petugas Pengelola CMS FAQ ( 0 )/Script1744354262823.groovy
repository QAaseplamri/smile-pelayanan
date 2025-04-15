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

CustomKeywords.'sectionMenu.utilityMenu.changeRoleSMILE'()

String inisial = findTestData('Data Files/Pengajuan FAQ/data_role').getValue('inisial', 5)

String role = findTestData('Data Files/Pengajuan FAQ/data_role').getValue('role', 5)

WebUI.callTestCase(findTestCase('01-Login/Login-pilihRoleSSMILE-02_Success'), [('inisial') : inisial, ('role') : role], 
    FailureHandling.STOP_ON_FAILURE)

CustomKeywords.'sectionMenu.utilityMenu.selectMenu'('e-Channel')

CustomKeywords.'sectionMenu.utilityMenu.selectMenu'('JMO')

CustomKeywords.'sectionMenu.utilityMenu.selectMenu'('AS1033-CMS FAQ Monitoring Approval')

not_run: WebUI.doubleClick(findTestObject('Object Repository/Konten faq/Pengajuan FAQ/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_e-Channel'))

not_run: WebUI.doubleClick(findTestObject('Object Repository/Konten faq/Pengajuan FAQ/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/span_JMO'))

not_run: WebUI.doubleClick(findTestObject('Konten faq/Pengajuan FAQ/Page_SMILE - Sistem Informasi Perlindungan _1e51ce/AS1033-CMS FAQ Monitoring Approval'))

WebUI.waitForElementVisible(findTestObject('Konten faq/Pengajuan FAQ/Data ke 1/Page_SMILE - Sistem Informasi Perlindungan Pekerja (46)/input_Kode Pengajuan_search_txt4'), 
    0)

// Input kode pengajuan ke dalam kolom pencarian
WebUI.click(findTestObject('Konten faq/Pengajuan FAQ/Data ke 1/Page_SMILE - Sistem Informasi Perlindungan Pekerja (46)/input_Kode Pengajuan_search_txt4'), 
    FailureHandling.STOP_ON_FAILURE)

WebUI.delay(2)

WebUI.setText(findTestObject('Konten faq/Pengajuan FAQ/Data ke 1/Page_SMILE - Sistem Informasi Perlindungan Pekerja (46)/input_Kode Pengajuan_search_txt4'), 
    GlobalVariable.kodePengajuan)

// Klik tombol cari
WebUI.click(findTestObject('Konten faq/Pengajuan FAQ/Data ke 1/Page_SMILE - Sistem Informasi Perlindungan Pekerja (46)/input_Kode Pengajuan_btnfilter'))

WebUI.delay(2)

WebUI.takeScreenshot()

