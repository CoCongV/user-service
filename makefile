outtest:
	go test -coverprofile=c.out -coverpkg user-service,user-service/apihandlers,user-service/apiv1,user-service/config,user-service/models,user-service/server

showcover:
	go tool cover -html=c.out