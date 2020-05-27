package main

//Run with:
// go run portscan.go 127.0.0.1

import (
	"fmt"
	"log"
	"net"
	"os"
	"strconv"
	"time"
)

func printUsage() {
	fmt.Println("Usage: ")
	fmt.Println("	go run portscan.go <host>")
	fmt.Println("Example: ")
	fmt.Println("	go run portscan.go 127.0.0.1")
}

func testTcpConnect(host string, port int, doneChannel chan bool) {
	timeoutLength := 5 * time.Second
	conn, err := net.DialTimeout("tcp", host+":"+strconv.Itoa(port), timeoutLength)
	if err != nil {
		doneChannel <- false
		return // could not connect
	}
	conn.Close()
	log.Printf("[+] %d connected", port)
	doneChannel <- true
}

func main() {
	if len(os.Args) == 1 {
		log.Println("No arguments provided.")
		printUsage()
		os.Exit(1)
	}

	doneChannel := make(chan bool)
	activeThreadCount := 0
	log.Println("Scanning host: " + os.Args[1])
	for portNumber := 1; portNumber <= 65535; portNumber++ {
		activeThreadCount++
		go testTcpConnect(os.Args[1], portNumber, doneChannel)
	}

	for {
		<-doneChannel
		activeThreadCount--
		if activeThreadCount == 0 {
			break
		}

	}
	// until activeThreadCount == 0 keep checking
	//

}
