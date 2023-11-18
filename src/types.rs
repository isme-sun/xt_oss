/*
HTTP/1.1 200 OK
x-oss-request-id: 3a8f-2e2d-7965-3ff9-51c875b*****
Date: Fri, 20 Aug 2021 06:40:30 GMT
Content-Type: application/xml
Content-Length: 3446
Server: AliyunOSS

<?xml version="1.0" encoding="UTF-8"?>
<RegionInfoList>
  <RegionInfo>
    <Region>oss-cn-hangzhou</Region>
    <InternetEndpoint>oss-cn-hangzhou.aliyuncs.com</InternetEndpoint>
    <InternalEndpoint>oss-cn-hangzhou-internal.aliyuncs.com</InternalEndpoint>
    <AccelerateEndpoint>oss-accelerate.aliyuncs.com</AccelerateEndpoint>  
  </RegionInfo>
</RegionInfoList>
*/

#[derive(Debug,Default)]
pub struct RegionInfo {
	pub region:String,
	pub internet_endpoint: String,
	pub internal_endpoint: String,
	pub accelerate_endpoint: String
}

pub type RegionInfoList = Vec<RegionInfo>;