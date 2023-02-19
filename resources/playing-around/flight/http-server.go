package flight

import (
	"fmt"
	"net/http"
)

func print(w http.ResponseWriter, req *http.Request) {
	fmt.Printf("%s\n", req.RequestURI)
	fmt.Fprintf(w, fmt.Sprintf("%s hey there\n", req.URL))
}

func hs() {
	http.HandleFunc("/", print)
	http.ListenAndServe(":8090", nil)
}
