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

/* Call a custom keyword to select the SMILE role with given parameters inisial and role.
 * @param inisial, admp for ADMP - Administrator ( 0 )
 * role for ADMP - Administrator ( 0 )
 * got initial or role from SMDF_roleSMILE at DATA Files Folder 
 * you can call this method using syntax 
 * WebUI.callTestCase(findTestCase('Test Cases/01-Login/00-Common/COM-Common-PilihRole_Success'), 
    [ 'inisial': 'ABC', 'role': 'Admin' ], 
    FailureHandling.STOP_ON_FAILURE)
 * */ 

CustomKeywords.'sectionLogin.pilihRole.pilihRoleSMILE'(inisial, role)

