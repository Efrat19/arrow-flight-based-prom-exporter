package flight

import (
	"fmt"
	"net/http"

	"context"
	"log"

	"github.com/apache/arrow/go/arrow/flight"
	"google.golang.org/grpc"
)

func proxy(w http.ResponseWriter, req *http.Request) {
	upstream := "localhost:0"
	fmt.Printf("%s\n", req.RequestURI)
	conn, err := grpc.Dial(upstream, grpc.WithInsecure())
	if err != nil {
		log.Fatal(err)
	}
	defer conn.Close()

	client := flight.NewFlightServiceClient(conn)
	stream, err := client.Handshake(context.Background())
	if err != nil {
		log.Fatal(err)
	}

	// ignore error handling here for brevity
	stream.Send(&flight.HandshakeRequest{Payload: []byte("request")})

	resp, _ := stream.Recv()
	fmt.Println(string(resp.Payload))
}

func hpfs() {
	http.HandleFunc("/", proxy)
	http.ListenAndServe(":8030", nil)
}
