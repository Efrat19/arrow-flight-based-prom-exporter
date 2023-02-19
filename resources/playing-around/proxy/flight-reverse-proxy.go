// // Licensed to the Apache Software Foundation (ASF) under one
// // or more contributor license agreements.  See the NOTICE file
// // distributed with this work for additional information
// // regarding copyright ownership.  The ASF licenses this file
// // to you under the Apache License, Version 2.0 (the
// // "License"); you may not use this file except in compliance
// // with the License.  You may obtain a copy of the License at
// //
// // http://www.apache.org/licenses/LICENSE-2.0
// //
// // Unless required by applicable law or agreed to in writing, software
// // distributed under the License is distributed on an "AS IS" BASIS,
// // WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// // See the License for the specific language governing permissions and
// // limitations under the License.


package main

import (
	"fmt"
	"net/http/httputil"
	"net/url"
	"strings"

	"github.com/labstack/echo"
	"context"
	"io"
	"log"

	"github.com/apache/arrow/go/arrow/flight"
	"google.golang.org/grpc"
	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/status"
)


// func main() {
// 	e := echo.New()

// 	// create the reverse proxy
// 	url, _ := url.Parse("http://localhost:8090")
// 	proxy := httputil.NewSingleHostReverseProxy(url)
// 	reverseProxyRoutePrefix := "/"
// 	routerGroup := e.Group(reverseProxyRoutePrefix)
// 	routerGroup.Use(func(handlerFunc echo.HandlerFunc) echo.HandlerFunc {
// 		return func(context echo.Context) error {

// 			req := context.Request()
// 			res := context.Response().Writer

// 			// Update the headers to allow for SSL redirection
// 			req.Host = url.Host
// 			req.URL.Host = url.Host
// 			req.URL.Scheme = url.Scheme

// 			//trim reverseProxyRoutePrefix
// 			path := req.URL.Path
// 			req.URL.Path = strings.TrimLeft(path, reverseProxyRoutePrefix)

// 			// ServeHttp is non blocking and uses a go routine under the hood
// 			fmt.Printf("%s\n", req.URL)
// 			proxy.ServeHTTP(res, req)
// 			return nil
// 		}
// 	})

// 	e.Start(":2957")

// }


