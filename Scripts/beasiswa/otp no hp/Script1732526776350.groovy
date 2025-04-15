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

import redisKeywords.RedisUtils as RedisUtils

List<Map> redisServers = [
	[('host') : '172.28.108.231', ('port') : 7000, ('password') : 'redisclusterDEV#'], // Server Redis pertama
	[('host') : '172.28.108.235', ('port') : 7000, ('password') : 'redisclusterDEV#'], // Server Redis kedua
	[('host') : '172.28.108.231', ('port') : 7001, ('password') : 'redisclusterDEV#']  // Server Redis ketiga
]

// Key yang digunakan untuk mendapatkan OTP
String smsotpkey = 'SMILE#AS1014#otp#0813815720003'

// Log untuk memastikan key benar
println('Key Redis yang dicari: ' + smsotpkey)

// Mendapatkan OTP dari beberapa Redis server
String otpsms = RedisUtils.getOTPFromMultipleRedisSMS(redisServers, smsotpkey)

if (otpsms != null) {
	println('OTP yang didapat dari Redis: ' + otpsms)
	Mobile.setText(findTestObject('Object Repository/Aktivasi Akun/PU/android.widget.EditText (1)'), otpsms, 2)
} else {
	println('Gagal mendapatkan OTP dari semua Redis servers.')
}