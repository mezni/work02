package main

import (
	"log"
	"math/rand"
	"os"
	"strconv"
	"time"
)

type TrxApp struct {
	TransportProtocol string
	AppProtocol       string
	AppName           string
	Domain            string
	Device            string
	Content           string
}

type TrxDates struct {
	StartDate int
	EndDate   int
	Duration  int
}

func generateTrxDates(startDate string, intervalMinutes int) TrxDates {
	var trxDates TrxDates
	beginIntervalDate, _ := time.Parse(timeFormat, startDate)
	endIntervalDate := beginIntervalDate.Add(time.Minute * time.Duration(intervalMinutes))
	beginIntervalDateTS := beginIntervalDate.Unix()
	endIntervalDateTS := endIntervalDate.Unix()
	startDateTS := int64(rand.Intn(int(endIntervalDateTS-beginIntervalDateTS))) + beginIntervalDateTS
	trxDuration := rand.Intn(trxDurationMax)
	trxDates.StartDate = int(startDateTS)
	trxDates.EndDate = int(startDateTS + int64(trxDuration))
	trxDates.Duration = trxDuration
	return trxDates
}

func generateTrxApp() TrxApp {
	var refTraProtocol = [2]string{"TCP", "UDP"}
	var refAppProtocol = [2]string{"https", "quic"}
	var refAppName = [5]string{"Youtube", "Facebook", "Google APIs", "Tiktok", "-"}
	var refDomain = [5]string{"youtubei.googleapi.com", "graph.facebook.com", "196.204.5.48",
		"142.250.185.106", "i.yting.com"}
	var refDevice = [5]string{"Samsung S22", "Samsung A54", "Iphone 14", "Iphone 14 pro", "Pixel"}
	var refContent = [4]string{"Web", "Video", "Text", "-"}

	var trxapp TrxApp
	trxapp.TransportProtocol = refTraProtocol[rand.Intn(len(refTraProtocol))]
	trxapp.AppProtocol = refAppProtocol[rand.Intn(len(refAppProtocol))]
	trxapp.AppName = refAppName[rand.Intn(len(refAppName))]
	trxapp.Domain = refDomain[rand.Intn(len(refDomain))]
	trxapp.Device = refDevice[rand.Intn(len(refDevice))]
	trxapp.Content = refContent[rand.Intn(len(refContent))]

	return trxapp
}

func getFileName(startDate string) (string, error) {
	beginIntervalDate, _ := time.Parse(timeFormat, startDate)
	beginIntervalDateTS := int(beginIntervalDate.Unix())
	fileName := "trans_" + strconv.Itoa(beginIntervalDateTS) + ".csv"
	return fileName, nil
}

func getFileTimestamp(startDate string) (int, error) {
	beginIntervalDate, _ := time.Parse(timeFormat, startDate)
	beginIntervalDateTS := int(beginIntervalDate.Unix())
	return beginIntervalDateTS, nil
}

func generateIP() string {
	ip := ""
	for i := 0; i <= 3; i++ {
		ip = ip + strconv.Itoa(rand.Intn(255)) + "."
	}
	ip = ip[0 : len(ip)-1]
	return ip
}

const timeFormat = "2006-01-02 15:04:05"
const trxDurationMax = 3600

func main() {
	log.Println("Start service")
	startDate := "2023-10-06 09:00:00"
	intervalMinutes := 5
	fileName, err := getFileName(startDate)
	fileTimestamp, err := getFileTimestamp(startDate)
	appInstance := strconv.Itoa(rand.Intn(1000) + 3000)
	appPID := strconv.Itoa(rand.Intn(10000) + 20000)

	f, err := os.OpenFile(fileName,
		os.O_APPEND|os.O_CREATE|os.O_WRONLY, 0644)
	if err != nil {
		log.Println(err)
	}
	defer f.Close()

	var columns = []string{"Timestamp", "type", "appName", "appInstance", "appPID",
		"transactionStart", "transactionEnd", "clientIPAddress", "clientPort",
		"serverIPAddress", "serverPort", "ipProtocol", "bytesToClient", "bytesFromClient",
		"bytesFromServer", "bytesToServer", "subsrciberID", "applicationProtocol",
		"applicationName", "domain", "deviceType", "transactionDuration", "contentType",
		"lostBytesClient", "lostBytesServer", "srttMsClient", "srttMsServer"}

	var header string = ""
	for _, col := range columns {
		header = header + "," + col
	}
	header = header[1:len(header)] + "\n"
	//	fmt.Printf("%s", header)
	_, err = f.WriteString(header)
	if err != nil {
		log.Println(err)
	}

	for i := 0; i < 1000000; i++ {
		var line string = ""
		trxApp := generateTrxApp()
		trxDates := generateTrxDates(startDate, intervalMinutes)
		bytesToClient := rand.Intn(10000)
		bytesFromClient := rand.Intn(10000)
		for _, col := range columns {
			switch col {
			case "Timestamp":
				line = line + "," + strconv.Itoa(fileTimestamp)
			case "type":
				line = line + "," + "AllIPMessages"
			case "appName":
				line = line + "," + trxApp.AppName
			case "appInstance":
				line = line + "," + appInstance
			case "appPID":
				line = line + "," + appPID
			case "transactionStart":
				line = line + "," + strconv.Itoa(trxDates.StartDate)
			case "transactionEnd":
				line = line + "," + strconv.Itoa(trxDates.EndDate)
			case "clientIPAddress":
				line = line + "," + generateIP()
			case "clientPort":
				line = line + "," + strconv.Itoa(rand.Intn(65000-1024)+1024)
			case "serverIPAddress":
				line = line + "," + generateIP()
			case "serverPort":
				line = line + "," + strconv.Itoa(443)
			case "ipProtocol":
				line = line + "," + trxApp.TransportProtocol
			case "bytesToClient":
				line = line + "," + strconv.Itoa(bytesToClient)
			case "bytesFromClient":
				line = line + "," + strconv.Itoa(bytesFromClient)
			case "bytesFromServer":
				line = line + "," + strconv.Itoa(bytesToClient)
			case "bytesToServer":
				line = line + "," + strconv.Itoa(bytesFromClient)
			case "subsrciberID":
				line = line + "," + strconv.Itoa(rand.Intn(10000)+2010123450000)
			case "applicationProtocol":
				line = line + "," + trxApp.AppProtocol
			case "applicationName":
				line = line + "," + trxApp.AppName
			case "domain":
				line = line + "," + trxApp.Domain
			case "deviceType":
				line = line + "," + trxApp.Device
			case "transactionDuration":
				line = line + "," + strconv.Itoa(trxDates.Duration)
			case "contentType":
				line = line + "," + trxApp.Content
			case "lostBytesClient":
				line = line + "," + strconv.Itoa(rand.Intn(255))
			case "lostBytesServer":
				line = line + "," + strconv.Itoa(rand.Intn(255))
			case "srttMsClient":
				line = line + "," + strconv.Itoa(rand.Intn(255))
			case "srttMsServer":
				line = line + "," + strconv.Itoa(rand.Intn(255))
			}
		}
		line = line[1:len(line)] + "\n"
		//		fmt.Printf("%s", line)
		_, err = f.WriteString(line)
		if err != nil {
			log.Println(err)
		}
	}
	log.Println("Stop service")
}
