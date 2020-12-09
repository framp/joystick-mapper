package gocv

import (
	"image/color"
	"testing"
)

func TestAKAZE(t *testing.T) {
	img := IMRead("images/face.jpg", IMReadColor)
	if img.Empty() {
		t.Error("Invalid Mat in AKAZE test")
	}
	defer img.Close()

	dst := NewMat()
	defer dst.Close()

	ak := NewAKAZE()
	defer ak.Close()

	kp := ak.Detect(img)
	if len(kp) < 512 {
		t.Errorf("Invalid KeyPoint array in AKAZE test: %d", len(kp))
	}

	mask := NewMat()
	defer mask.Close()

	kp2, desc := ak.DetectAndCompute(img, mask)
	if len(kp2) < 512 {
		t.Errorf("Invalid KeyPoint array in AKAZE DetectAndCompute: %d", len(kp2))
	}

	if desc.Empty() {
		t.Error("Invalid Mat desc in AKAZE DetectAndCompute")
	}
}

func TestAgastFeatureDetector(t *testing.T) {
	img := IMRead("images/face.jpg", IMReadColor)
	if img.Empty() {
		t.Error("Invalid Mat in AgastFeatureDetector test")
	}
	defer img.Close()

	dst := NewMat()
	defer dst.Close()

	ad := NewAgastFeatureDetector()
	defer ad.Close()

	kp := ad.Detect(img)
	if len(kp) < 2800 {
		t.Errorf("Invalid KeyPoint array in AgastFeatureDetector test: %d", len(kp))
	}
}

func TestBRISK(t *testing.T) {
	img := IMRead("images/face.jpg", IMReadColor)
	if img.Empty() {
		t.Error("Invalid Mat in BRISK test")
	}
	defer img.Close()

	dst := NewMat()
	defer dst.Close()

	br := NewBRISK()
	defer br.Close()

	kp := br.Detect(img)
	if len(kp) < 513 {
		t.Errorf("Invalid KeyPoint array in BRISK Detect: %d", len(kp))
	}

	mask := NewMat()
	defer mask.Close()

	kp2, desc := br.DetectAndCompute(img, mask)
	if len(kp2) != 1105 {
		t.Errorf("Invalid KeyPoint array in BRISK DetectAndCompute: %d", len(kp2))
	}

	if desc.Empty() {
		t.Error("Invalid Mat desc in AKAZE DetectAndCompute")
	}
}

func TestFastFeatureDetector(t *testing.T) {
	img := IMRead("images/face.jpg", IMReadColor)
	if img.Empty() {
		t.Error("Invalid Mat in FastFeatureDetector test")
	}
	defer img.Close()

	dst := NewMat()
	defer dst.Close()

	fd := NewFastFeatureDetector()
	defer fd.Close()

	kp := fd.Detect(img)
	if len(kp) < 2690 {
		t.Errorf("Invalid KeyPoint array in FastFeatureDetector test: %d", len(kp))
	}
}

func TestGFTTDetector(t *testing.T) {
	img := IMRead("images/face.jpg", IMReadColor)
	if img.Empty() {
		t.Error("Invalid Mat in GFTTDetector test")
	}
	defer img.Close()

	dst := NewMat()
	defer dst.Close()

	gft := NewGFTTDetector()
	defer gft.Close()

	kp := gft.Detect(img)
	if len(kp) < 512 {
		t.Errorf("Invalid KeyPoint array in GFTTDetector test: %d", len(kp))
	}
}

func TestKAZE(t *testing.T) {
	img := IMRead("images/face.jpg", IMReadColor)
	if img.Empty() {
		t.Error("Invalid Mat in KAZE test")
	}
	defer img.Close()

	dst := NewMat()
	defer dst.Close()

	k := NewKAZE()
	defer k.Close()

	kp := k.Detect(img)
	if len(kp) < 512 {
		t.Errorf("Invalid KeyPoint array in KAZE test: %d", len(kp))
	}

	mask := NewMat()
	defer mask.Close()

	kp2, desc := k.DetectAndCompute(img, mask)
	if len(kp2) < 512 {
		t.Errorf("Invalid KeyPoint array in KAZE DetectAndCompute: %d", len(kp2))
	}

	if desc.Empty() {
		t.Error("Invalid Mat desc in KAZE DetectAndCompute")
	}
}

func TestMSER(t *testing.T) {
	img := IMRead("images/face.jpg", IMReadColor)
	if img.Empty() {
		t.Error("Invalid Mat in MSER test")
	}
	defer img.Close()

	dst := NewMat()
	defer dst.Close()

	mser := NewMSER()
	defer mser.Close()

	kp := mser.Detect(img)
	if len(kp) != 234 && len(kp) != 261 {
		t.Errorf("Invalid KeyPoint array in MSER test: %d", len(kp))
	}
}

func TestORB(t *testing.T) {
	img := IMRead("images/face.jpg", IMReadColor)
	if img.Empty() {
		t.Error("Invalid Mat in AgastFeatureDetector test")
	}
	defer img.Close()

	dst := NewMat()
	defer dst.Close()

	od := NewORB()
	defer od.Close()

	kp := od.Detect(img)
	if len(kp) != 500 {
		t.Errorf("Invalid KeyPoint array in ORB test: %d", len(kp))
	}

	mask := NewMat()
	defer mask.Close()

	kp2, desc := od.DetectAndCompute(img, mask)
	if len(kp2) != 500 {
		t.Errorf("Invalid KeyPoint array in ORB DetectAndCompute: %d", len(kp2))
	}

	if desc.Empty() {
		t.Error("Invalid Mat desc in ORB DetectAndCompute")
	}
}

func TestSimpleBlobDetector(t *testing.T) {
	img := IMRead("images/face.jpg", IMReadColor)
	if img.Empty() {
		t.Error("Invalid Mat in SimpleBlobDetector test")
	}
	defer img.Close()

	dst := NewMat()
	defer dst.Close()

	bd := NewSimpleBlobDetector()
	defer bd.Close()

	kp := bd.Detect(img)
	if len(kp) != 2 {
		t.Errorf("Invalid KeyPoint array in SimpleBlobDetector test: %d", len(kp))
	}
}

func TestBFMatcher(t *testing.T) {
	descriptorFile := "images/sift_descriptor.png"
	desc1 := IMRead(descriptorFile, IMReadGrayScale)
	if desc1.Empty() {
		t.Error("descriptor one is empty in BFMatcher test")
	}
	defer desc1.Close()

	desc2 := IMRead(descriptorFile, IMReadGrayScale)
	if desc2.Empty() {
		t.Error("descriptor two is empty in BFMatcher test")
	}
	defer desc2.Close()

	bf := NewBFMatcher()
	defer bf.Close()

	k := 2
	dMatches := bf.KnnMatch(desc1, desc2, k)
	if len(dMatches) < 1 {
		t.Errorf("DMatches was excepted to have at least one element")
	}
	for i := range dMatches {
		if len(dMatches[i]) != k {
			t.Errorf("Length does not match k cluster amount in BFMatcher")
		}
	}

	bfParams := NewBFMatcherWithParams(NormHamming, false)
	defer bfParams.Close()

	dMatches = bfParams.KnnMatch(desc1, desc2, k)
	if len(dMatches) < 1 {
		t.Errorf("DMatches was excepted to have at least one element")
	}
	for i := range dMatches {
		if len(dMatches[i]) != k {
			t.Errorf("Length does not match k cluster amount in BFMatcher")
		}
	}
}

func TestDrawKeyPoints(t *testing.T) {
	keypointsFile := "images/simple.jpg"
	img := IMRead(keypointsFile, IMReadColor)
	if img.Empty() {
		t.Error("keypoints file is empty in DrawKeyPoints test")
	}
	defer img.Close()

	ffd := NewFastFeatureDetector()
	kp := ffd.Detect(img)

	simpleKP := NewMat()
	DrawKeyPoints(img, kp, &simpleKP, color.RGBA{255, 0, 0, 0}, DrawDefault)

	if simpleKP.Rows() != img.Rows() || simpleKP.Cols() != img.Cols() {
		t.Error("Invalid DrawKeyPoints test")
	}
}
