package handlers

import (
	"encoding/json"
	"math/rand"
	"net/http"
	"strconv"
	"time"
)

type Event struct {
	Timestamp           string `json:"timestamp"`
	Type                string `json:"type"`
	AppName             string `json:"appName"`
	AppInstance         string `json:"appInstance"`
	AppPID              string `json:"appPID"`
	TransactionStart    string `json:"transactionStart"`
	TransactionEnd      string `json:"transactionEnd"`
	ClientIPAddress     string `json:"clientIPAddress"`
	ClientPort          string `json:"clientPort"`
	ServerIPAddress     string `json:"serverIPAddress"`
	ServerPort          string `json:"serverPort"`
	IpProtocol          string `json:"ipProtocol"`
	BytesToClient       string `json:"bytesToClient"`
	BytesFromClient     string `json:"bytesFromClient"`
	BytesFromServer     string `json:"bytesFromServer"`
	BytesToServer       string `json:"bytesToServer"`
	SubsrciberID        string `json:"subsrciberID"`
	ApplicationProtocol string `json:"applicationProtocol"`
	ApplicationName     string `json:"applicationName"`
	Domain              string `json:"domain"`
	DeviceType          string `json:"deviceType"`
	TransactionDuration string `json:"transactionDuration"`
	ContentType         string `json:"contentType"`
	LostBytesClient     string `json:"lostBytesClient"`
	LostBytesServer     string `json:"lostBytesServer"`
	SrttMsClient        string `json:"srttMsClient"`
	SrttMsServer        string `json:"srttMsServer"`
}

type InRequest struct {
	IntervalStartDate string `json:"IntervalStartDate"`
	IntervalMinutes   int    `json:"intervalMinutes"`
	TrxCount          int    `json:"trxCount"`
}

type AppInfo struct {
	Timestamp         string
	Type              string
	AppName           string
	AppInstance       string
	AppPID            string
	TransportProtocol string
	AppProtocol       string
	Domain            string
	Device            string
	Content           string
	ApplicationName   string
}

type TrxInfo struct {
	StartDate       string
	EndDate         string
	Duration        string
	BytesToClient   string
	BytesFromClient string
	BytesFromServer string
	BytesToServer   string
	LostBytesClient string
	LostBytesServer string
	SrttMsClient    string
	SrttMsServer    string
	SubsrciberID    string
}

func generateIP() string {
	ip := ""
	for i := 0; i <= 3; i++ {
		ip = ip + strconv.Itoa(rand.Intn(255)) + "."
	}
	ip = ip[0 : len(ip)-1]
	return ip
}

func generateAppInfo(startDate time.Time) AppInfo {
	var refTraProtocol = []string{"TCP", "UDP"}
	var refAppProtocol = []string{"https", "quic"}
	var refAppName = []string{"Youtube", "Facebook", "Google APIs", "Tiktok", "-"}
	var refDomain = []string{"youtubei.googleapi.com", "graph.facebook.com", "196.204.5.48",
		"142.250.185.106", "i.yting.com"}
	var refDevice = []string{"Samsung S22", "Samsung A54", "Iphone 14", "Iphone 14 pro", "Pixel"}
	var refContent = []string{"Web", "Video", "Text", "-"}

	var appInfo AppInfo
	appInfo.TransportProtocol = refTraProtocol[rand.Intn(len(refTraProtocol))]
	appInfo.AppProtocol = refAppProtocol[rand.Intn(len(refAppProtocol))]
	appInfo.ApplicationName = refAppName[rand.Intn(len(refAppName))]
	appInfo.Domain = refDomain[rand.Intn(len(refDomain))]
	appInfo.Device = refDevice[rand.Intn(len(refDevice))]
	appInfo.Content = refContent[rand.Intn(len(refContent))]

	appInfo.Timestamp = strconv.Itoa(int(startDate.Unix()))
	appInfo.Type = "AllIPMessages"
	appInfo.AppName = "TraficServerElement"
	appInfo.AppInstance = strconv.Itoa(int(startDate.Unix()))[3:7]
	appInfo.AppPID = strconv.Itoa(rand.Intn(55000) + 1000)

	return appInfo
}

func generateTrxInfo(startDate time.Time, intervalMinutes int) TrxInfo {
	var trxInfo TrxInfo
	startDateTS := int(startDate.Unix())
	endDate := startDate.Add(time.Minute * time.Duration(intervalMinutes))
	endDateTS := int(endDate.Unix())
	trxDuration := rand.Intn(trxDurationMax)
	trxEndDateTS := int(rand.Intn(int(endDateTS-startDateTS))) + int(startDateTS) //int64(rand.Intn(int(endDateTS-startDateTS))) + startDateTS
	trxStartDateTS := int(startDateTS) - trxDuration

	trxInfo.StartDate = strconv.Itoa(trxStartDateTS)
	trxInfo.EndDate = strconv.Itoa(trxEndDateTS)
	trxInfo.Duration = strconv.Itoa(trxDuration)

	bytesToClient := rand.Intn(10000)
	bytesFromClient := rand.Intn(10000)
	trxInfo.BytesToClient = strconv.Itoa(bytesToClient)
	trxInfo.BytesFromClient = strconv.Itoa(bytesFromClient)
	trxInfo.BytesFromServer = strconv.Itoa(bytesFromClient)
	trxInfo.BytesToServer = strconv.Itoa(bytesToClient)

	trxInfo.LostBytesClient = strconv.Itoa(rand.Intn(512))
	trxInfo.LostBytesServer = strconv.Itoa(rand.Intn(512))
	trxInfo.SrttMsClient = strconv.Itoa(rand.Intn(512))
	trxInfo.SrttMsServer = strconv.Itoa(rand.Intn(512))

	trxInfo.SubsrciberID = strconv.Itoa(rand.Intn(10000) + 2010123450000)
	return trxInfo
}

const timeFormat = "2006-01-02 15:04:05"
const trxDurationMax = 3600

func generateEvents(intervalStartDate string, intervalMinutes int, trxCount int) ([]Event, error) {
	var events []Event
	startDate, err := time.Parse(timeFormat, intervalStartDate)
	if err != nil {
		return events, err
	}

	trxInfo := generateTrxInfo(startDate, intervalMinutes)
	appInfo := generateAppInfo(startDate)
	for i := 0; i < trxCount; i++ {
		event := Event{}
		event.TransactionStart = trxInfo.StartDate
		event.TransactionEnd = trxInfo.EndDate
		event.TransactionDuration = trxInfo.EndDate
		event.BytesToClient = trxInfo.BytesToClient
		event.BytesFromClient = trxInfo.BytesFromClient
		event.BytesFromServer = trxInfo.BytesFromServer
		event.BytesToServer = trxInfo.BytesToServer
		event.LostBytesClient = trxInfo.LostBytesClient
		event.LostBytesServer = trxInfo.LostBytesServer
		event.SrttMsClient = trxInfo.SrttMsClient
		event.SrttMsServer = trxInfo.SrttMsServer

		event.ClientIPAddress = generateIP()
		event.ClientPort = strconv.Itoa(rand.Intn(46000) + 1024)
		event.ServerIPAddress = generateIP()
		event.ServerPort = strconv.Itoa(443)

		event.Timestamp = appInfo.Timestamp
		event.IpProtocol = appInfo.TransportProtocol
		event.ApplicationProtocol = appInfo.AppProtocol
		event.ApplicationName = appInfo.ApplicationName
		event.Domain = appInfo.Domain
		event.DeviceType = appInfo.Device
		event.ContentType = appInfo.Content

		event.Type = appInfo.Type
		event.AppName = appInfo.AppName
		event.AppInstance = appInfo.AppInstance 
		event.AppPID = appInfo.AppPID
	
		events = append(events, event)
	}

	return events, nil
}

func HealthHandler(w http.ResponseWriter, r *http.Request) {
	json.NewEncoder(w).Encode(map[string]bool{"ok": true})
}

func GenerateEventsHandler(w http.ResponseWriter, r *http.Request) {
	w.Header().Set("Content-Type", "application/json")
	var inRequest InRequest
	err := json.NewDecoder(r.Body).Decode(&inRequest)

	if err != nil {
		w.WriteHeader(http.StatusBadRequest)
		return
	}
	intervalStartDate := inRequest.IntervalStartDate
	intervalMinutes := inRequest.IntervalMinutes
	trxCount := inRequest.TrxCount
	events, _ := generateEvents(intervalStartDate, intervalMinutes, trxCount)
	json.NewEncoder(w).Encode(events)

}
