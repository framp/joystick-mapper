# Using the Intel OpenVINO Inference Engine

The Intel OpenVINO Inference Engine is a set of libraries for executing convolutional neural networks.

GoCV support for the Intel OpenVINO Inference Engine will be able to be found here in the "gocv.io/x/gocv/openvino/ie" package.

## How It Works

Support in GoCV for the Intel OpenVINO Inference Engine requires version R2+ in order to work.

## How to use

This code loads a Caffe model, and then uses OpenVINO inference engine to prepare it for execution on the GPU:

```go
net := gocv.ReadNet("/path/to/your/model.caffemodel", "/path/to/your/config.proto")
if net.Empty() {
    fmt.Println("Error reading network model")
    return
}

net.SetPreferableBackend(gocv.NetBackendType(gocv.NetBackendOpenVINO))
net.SetPreferableTarget(gocv.NetTargetType(gocv.NetBackendFP16))
```
