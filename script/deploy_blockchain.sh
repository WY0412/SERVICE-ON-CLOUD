aws ecr get-login-password --region us-east-1 | docker login --username AWS  --password-stdin 851725245278.dkr.ecr.us-east-1.amazonaws.com
# aws ecr create-repository     --repository-name team-project-ecr --region us-east-1
docker build -f web/web_ntex/Dockerfile -t 851725245278.dkr.ecr.us-east-1.amazonaws.com/team-project-ecr web/web_ntex/
# docker run -p 8080:8080  851725245278.dkr.ecr.us-east-1.amazonaws.com/team-project-ecr
docker push 851725245278.dkr.ecr.us-east-1.amazonaws.com/team-project-ecr
kubectl apply -f Ingress/ingress.yaml 
helm install blockchain k8s/helm/blockchain
kubectl get ingress
kubectl get svc
kubectl get pods
# by using kubectl top pod / kubectl top node kubectl describe pods kubectl describe nodes
# helm uninstall blockchain