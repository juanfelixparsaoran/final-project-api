<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Update booking</name>
   <tag></tag>
   <elementGuidId>5b7af224-b281-4edd-a010-13840c5f1604</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;firstname\&quot; : \&quot;James\&quot;,\n    \&quot;lastname\&quot; : \&quot;Brown\&quot;,\n    \&quot;totalprice\&quot; : 111,\n    \&quot;depositpaid\&quot; : true,\n    \&quot;bookingdates\&quot; : {\n        \&quot;checkin\&quot; : \&quot;2018-01-01\&quot;,\n        \&quot;checkout\&quot; : \&quot;2019-01-01\&quot;\n    },\n    \&quot;additionalneeds\&quot; : \&quot;Lunch\&quot;\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>3649d8ef-04e0-45a2-a16f-330876ad619c</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Accept</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>1dd9a364-3d58-4afd-aca1-b58b21682a32</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Basic YWRtaW46cGFzc3dvcmQxMjM=</value>
      <webElementGuid>340a343e-f6f9-4874-8097-cf328e047225</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.3.5</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>PUT</restRequestMethod>
   <restUrl>https://restful-booker.herokuapp.com/booking/3673</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()
WS.verifyElementPropertyValue(response, 'firstname', &quot;James&quot;)
WS.verifyElementPropertyValue(response, 'lastname', &quot;Brown&quot;)
WS.verifyElementPropertyValue(response, 'totalprice', 111)
WS.verifyElementPropertyValue(response, 'depositpaid', true)
WS.verifyElementPropertyValue(response, 'bookingdates.checkin', &quot;2018-01-01&quot;)
WS.verifyElementPropertyValue(response, 'bookingdates.checkout', &quot;2019-01-01&quot;)
WS.verifyElementPropertyValue(response, 'additionalneeds', &quot;Lunch&quot;)


WS.verifyResponseStatusCode(response, 200)

assertThat(response.getStatusCode()).isEqualTo(200)</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
