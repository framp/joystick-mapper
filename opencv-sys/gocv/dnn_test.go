package gocv

import (
	"image"
	"os"
	"testing"
)

func TestReadNet(t *testing.T) {
	path := os.Getenv("GOCV_CAFFE_TEST_FILES")
	if path == "" {
		t.Skip("Unable to locate Caffe model files for tests")
	}

	net := ReadNet(path+"/bvlc_googlenet.caffemodel", path+"/bvlc_googlenet.prototxt")
	if net.Empty() {
		t.Errorf("Unable to load Caffe model using ReadNet")
	}
	defer net.Close()

	net.SetPreferableBackend(NetBackendDefault)
	net.SetPreferableTarget(NetTargetCPU)

	img := IMRead("images/space_shuttle.jpg", IMReadColor)
	if img.Empty() {
		t.Error("Invalid Mat in ReadNet test")
	}
	defer img.Close()

	blob := BlobFromImage(img, 1.0, image.Pt(224, 224), NewScalar(104, 117, 123, 0), false, false)
	if blob.Empty() {
		t.Error("Invalid blob in ReadNet test")
	}
	defer blob.Close()

	net.SetInput(blob, "data")

	layer := net.GetLayer(0)
	defer layer.Close()

	if layer.InputNameToIndex("notthere") != -1 {
		t.Error("Invalid layer in ReadNet test")
	}
	if layer.OutputNameToIndex("notthere") != -1 {
		t.Error("Invalid layer in ReadNet test")
	}
	if layer.GetName() != "_input" {
		t.Errorf("Invalid layer name in ReadNet test: %s\n", layer.GetName())
	}
	if layer.GetType() != "" {
		t.Errorf("Invalid layer type in ReadNet test: %s\n", layer.GetType())
	}

	ids := net.GetUnconnectedOutLayers()
	if len(ids) != 1 {
		t.Errorf("Invalid len output layers in ReadNet test: %d\n", len(ids))
	}

	prob := net.ForwardLayers([]string{"prob"})
	if len(prob) == 0 {
		t.Error("Invalid len prob in ReadNet test")
	}

	if prob[0].Empty() {
		t.Error("Invalid prob[0] in ReadNet test")
	}

	probMat := prob[0].Reshape(1, 1)
	_, maxVal, minLoc, maxLoc := MinMaxLoc(probMat)

	if round(float64(maxVal), 0.00005) != 0.99995 {
		t.Errorf("ReadNet maxVal incorrect: %v\n", round(float64(maxVal), 0.00005))
	}

	if minLoc.X != 793 || minLoc.Y != 0 {
		t.Errorf("ReadNet minLoc incorrect: %v\n", minLoc)
	}

	if maxLoc.X != 812 || maxLoc.Y != 0 {
		t.Errorf("ReadNet maxLoc incorrect: %v\n", maxLoc)
	}

	perf := net.GetPerfProfile()
	if perf == 0 {
		t.Error("ReadNet GetPerfProfile error")
	}
}

func TestCaffe(t *testing.T) {
	path := os.Getenv("GOCV_CAFFE_TEST_FILES")
	if path == "" {
		t.Skip("Unable to locate Caffe model files for tests")
	}

	net := ReadNetFromCaffe(path+"/bvlc_googlenet.prototxt", path+"/bvlc_googlenet.caffemodel")
	if net.Empty() {
		t.Errorf("Unable to load Caffe model")
	}
	defer net.Close()

	img := IMRead("images/space_shuttle.jpg", IMReadColor)
	if img.Empty() {
		t.Error("Invalid Mat in Caffe test")
	}
	defer img.Close()

	blob := BlobFromImage(img, 1.0, image.Pt(224, 224), NewScalar(104, 117, 123, 0), false, false)
	if blob.Empty() {
		t.Error("Invalid blob in Caffe test")
	}
	defer blob.Close()

	net.SetInput(blob, "data")
	prob := net.Forward("prob")
	if prob.Empty() {
		t.Error("Invalid prob in Caffe test")
	}

	probMat := prob.Reshape(1, 1)
	_, maxVal, minLoc, maxLoc := MinMaxLoc(probMat)

	if round(float64(maxVal), 0.00005) != 0.99995 {
		t.Errorf("Caffe maxVal incorrect: %v\n", round(float64(maxVal), 0.00005))
	}

	if minLoc.X != 793 || minLoc.Y != 0 {
		t.Errorf("Caffe minLoc incorrect: %v\n", minLoc)
	}

	if maxLoc.X != 812 || maxLoc.Y != 0 {
		t.Errorf("Caffe maxLoc incorrect: %v\n", maxLoc)
	}
}

func TestTensorflow(t *testing.T) {
	path := os.Getenv("GOCV_TENSORFLOW_TEST_FILES")
	if path == "" {
		t.Skip("Unable to locate Tensorflow model file for tests")
	}

	net := ReadNetFromTensorflow(path + "/tensorflow_inception_graph.pb")
	if net.Empty() {
		t.Errorf("Unable to load Tensorflow model")
	}
	defer net.Close()

	img := IMRead("images/space_shuttle.jpg", IMReadColor)
	if img.Empty() {
		t.Error("Invalid Mat in Tensorflow test")
	}
	defer img.Close()

	blob := BlobFromImage(img, 1.0, image.Pt(224, 224), NewScalar(0, 0, 0, 0), true, false)
	if blob.Empty() {
		t.Error("Invalid blob in Tensorflow test")
	}
	defer blob.Close()

	net.SetInput(blob, "input")
	prob := net.Forward("softmax2")
	if prob.Empty() {
		t.Error("Invalid softmax2 in Tensorflow test")
	}

	probMat := prob.Reshape(1, 1)
	_, maxVal, minLoc, maxLoc := MinMaxLoc(probMat)

	if round(float64(maxVal), 0.00005) != 1.0 {
		t.Errorf("Tensorflow maxVal incorrect: %v\n", round(float64(maxVal), 0.00005))
	}

	if minLoc.X != 481 || minLoc.Y != 0 {
		t.Errorf("Tensorflow minLoc incorrect: %v\n", minLoc)
	}

	if maxLoc.X != 234 || maxLoc.Y != 0 {
		t.Errorf("Tensorflow maxLoc incorrect: %v\n", maxLoc)
	}
}

func TestGetBlobChannel(t *testing.T) {
	img := NewMatWithSize(100, 100, 5+16)
	defer img.Close()

	blob := BlobFromImage(img, 1.0, image.Pt(0, 0), NewScalar(0, 0, 0, 0), true, false)
	defer blob.Close()

	ch2 := GetBlobChannel(blob, 0, 1)
	defer ch2.Close()

	if ch2.Empty() {
		t.Errorf("GetBlobChannel failed to retrieve 2nd chan of a 3channel blob")
	}
	if ch2.Rows() != img.Rows() || ch2.Cols() != img.Cols() {
		t.Errorf("GetBlobChannel: retrieved image size does not match original")
	}
}

func TestGetBlobSize(t *testing.T) {
	img := NewMatWithSize(100, 100, 5+16)
	defer img.Close()

	blob := BlobFromImage(img, 1.0, image.Pt(0, 0), NewScalar(0, 0, 0, 0), true, false)
	defer blob.Close()

	sz := GetBlobSize(blob)
	if sz.Val1 != 1 || sz.Val2 != 3 || sz.Val3 != 100 || sz.Val4 != 100 {
		t.Errorf("GetBlobSize retrieved wrong values")
	}
}

func TestParseNetBackend(t *testing.T) {
	val := ParseNetBackend("halide")
	if val != NetBackendHalide {
		t.Errorf("ParseNetBackend invalid")
	}
	val = ParseNetBackend("openvino")
	if val != NetBackendOpenVINO {
		t.Errorf("ParseNetBackend invalid")
	}
	val = ParseNetBackend("opencv")
	if val != NetBackendOpenCV {
		t.Errorf("ParseNetBackend invalid")
	}
	val = ParseNetBackend("crazytrain")
	if val != NetBackendDefault {
		t.Errorf("ParseNetBackend invalid")
	}
}

func TestParseNetTarget(t *testing.T) {
	val := ParseNetTarget("cpu")
	if val != NetTargetCPU {
		t.Errorf("ParseNetTarget invalid")
	}
	val = ParseNetTarget("fp32")
	if val != NetTargetFP32 {
		t.Errorf("ParseNetTarget invalid")
	}
	val = ParseNetTarget("fp16")
	if val != NetTargetFP16 {
		t.Errorf("ParseNetTarget invalid")
	}
	val = ParseNetTarget("vpu")
	if val != NetTargetVPU {
		t.Errorf("ParseNetTarget invalid")
	}
	val = ParseNetTarget("idk")
	if val != NetTargetCPU {
		t.Errorf("ParseNetTarget invalid")
	}
}

func TestFP16BlobFromImage(t *testing.T) {
	img := NewMatWithSize(100, 100, 5+16)
	defer img.Close()

	data := FP16BlobFromImage(img, 1.0, image.Pt(100, 100), 0, false, false)

	if len(data) != 60000 {
		t.Errorf("FP16BlobFromImage incorrect length: %v\n", len(data))
	}
}
