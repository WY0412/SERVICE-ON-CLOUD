# export KOPS_STATE_STORE=s3://<bucket-name>

# write check for env variables
#if [ -z "$KOPS_STATE_STORE" ]; then
#    echo "KOPS_STATE_STORE is not set"
#    exit 1
#fi

#kops create -f k8s/cluster/cluster.yaml,k8s/cluster/ig-master.yaml,k8s/cluster/ig-nodes.yaml
#eksctl create cluster -f k8s/cluster/cluster.yaml
# kops edit cluster cluster.k8s.local
# Add sshKeyName WITHOUT .pem to the spec section so it looks like:
# spec:
#   sshKeyName: teamproject
#   additionalPolicies: ...


helm repo add eks https://aws.github.io/eks-charts
# helm repo add bitnami https://charts.bitnami.com/bitnami
helm repo update


CLUSTER_NAME=phase3-cluster
TEAM_AWS_ID=851725245278

eksctl utils associate-iam-oidc-provider --region us-east-1 --cluster $CLUSTER_NAME --approve
eksctl create iamserviceaccount --cluster=$CLUSTER_NAME --namespace=kube-system --name=aws-load-balancer-controller --attach-policy-arn=arn:aws:iam::$TEAM_AWS_ID:policy/AWSLoadBalancerControllerIAMPolicy --override-existing-serviceaccounts --approve

kubectl apply -k "github.com/aws/eks-charts/stable/aws-load-balancer-controller/crds?ref=master"

helm repo add eks https://aws.github.io/eks-charts

helm upgrade -i aws-load-balancer-controller eks/aws-load-balancer-controller --set clusterName=$CLUSTER_NAME --set serviceAccount.create=false --set region=us-east-1 --set serviceAccount.name=aws-load-balancer-controller -n kube-system

# Verify the intallation is successful by checking there is a pod named aws-load-balancer-controller running
kubectl get pods -A

#helm install aws-load-balancer-controller eks/aws-load-balancer-controller -n kube-system --set clusterName=cluster.k8s.local

# helm install mysql-profile --set auth.rootPassword=${MYSQL_PASSWORD},auth.username=${MYSQL_USER},auth.password=${MYSQL_PASSWORD},auth.database=test bitnami/mysql --set image.debug=true \
# --set primary.persistence.enabled=false,secondary.persistence.enabled=false \
# --set primary.readinessProbe.enabled=false,primary.livenessProbe.enabled=false \
# --set secondary.readinessProbe.enabled=false,secondary.livenessProbe.enabled=false
# wget https://raw.githubusercontent.com/kubernetes-sigs/aws-load-balancer-controller/v2.6.0/docs/install/iam_policy.json
# aws iam create-policy --policy-name alb-controller-policy --policy-document file://iam_policy.json

#aws iam attach-role-policy --role-name nodes.cluster.k8s.local --policy-arn arn:aws:iam::851725245278:policy/alb-controller-policy

# Install Amazon EBS CSI Driver and Snapshot Controller

# Add the AWS EBS CSI Driver Helm repository.

helm repo add aws-ebs-csi-driver https://kubernetes-sigs.github.io/aws-ebs-csi-driver
helm repo update

# Install the snapshot controller.
kubectl apply -f https://raw.githubusercontent.com/kubernetes-csi/external-snapshotter/master/client/config/crd/snapshot.storage.k8s.io_volumesnapshotclasses.yaml
kubectl apply -f https://raw.githubusercontent.com/kubernetes-csi/external-snapshotter/master/client/config/crd/snapshot.storage.k8s.io_volumesnapshotcontents.yaml
kubectl apply -f https://raw.githubusercontent.com/kubernetes-csi/external-snapshotter/master/client/config/crd/snapshot.storage.k8s.io_volumesnapshots.yaml
kubectl apply -f https://raw.githubusercontent.com/kubernetes-csi/external-snapshotter/master/deploy/kubernetes/snapshot-controller/rbac-snapshot-controller.yaml
kubectl apply -f https://raw.githubusercontent.com/kubernetes-csi/external-snapshotter/master/deploy/kubernetes/snapshot-controller/setup-snapshot-controller.yaml

kubectl apply -f https://raw.githubusercontent.com/kubernetes-csi/external-snapshotter/master/deploy/kubernetes/csi-snapshotter/rbac-csi-snapshotter.yaml
kubectl apply -f https://raw.githubusercontent.com/kubernetes-csi/external-snapshotter/master/deploy/kubernetes/csi-snapshotter/rbac-external-provisioner.yaml
kubectl apply -f https://raw.githubusercontent.com/kubernetes-csi/external-snapshotter/master/deploy/kubernetes/csi-snapshotter/setup-csi-snapshotter.yaml
# Install the EBS CSI Driver. The controller.extraVolumeTags value denotes the tags that will be attached to the EBS volumes that the driver provisions, so make sure you update it in later phases.

helm upgrade --install aws-ebs-csi-driver --namespace kube-system --set "controller.extraVolumeTags.Project=twitter-phase-3" aws-ebs-csi-driver/aws-ebs-csi-driver
# Attach the IAM policy for AWS EBS CSI Driver to the nodes' IAM role. Note that if you have changed your cluster name, you also need to make sure --role-name is correctly set.

#aws iam attach-role-policy --role-name nodes.cluster.k8s.local --policy-arn arn:aws:iam::aws:policy/service-role/AmazonEBSCSIDriverPolicy