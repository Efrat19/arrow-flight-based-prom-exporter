package main

import (
	"fmt"
	"net/http/httputil"
	"net/url"
	"strings"

	"github.com/labstack/echo"
)

func main1() {
	e := echo.New()

	// create the reverse proxy
	url, _ := url.Parse("http://localhost:8090")
	proxy := httputil.NewSingleHostReverseProxy(url)
	reverseProxyRoutePrefix := "/"
	routerGroup := e.Group(reverseProxyRoutePrefix)
	routerGroup.Use(func(handlerFunc echo.HandlerFunc) echo.HandlerFunc {
		return func(context echo.Context) error {

			req := context.Request()
			res := context.Response().Writer

			// Update the headers to allow for SSL redirection
			req.Host = url.Host
			req.URL.Host = url.Host
			req.URL.Scheme = url.Scheme

			//trim reverseProxyRoutePrefix
			path := req.URL.Path
			req.URL.Path = strings.TrimLeft(path, reverseProxyRoutePrefix)

			// ServeHttp is non blocking and uses a go routine under the hood
			fmt.Printf("%s\n", req.URL)
			proxy.ServeHTTP(res, req)
			return nil
		}
	})

	e.Start(":2957")

}
