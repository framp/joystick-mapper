#ifndef _OPENCV3_CUDA_H_
#define _OPENCV3_CUDA_H_

#include <stddef.h>
#include "gocv/core.h"

#ifdef __cplusplus
#include <opencv2/cudaobjdetect.hpp>
extern "C" {
#endif

// =============================================================================
//   GpuMat
// =============================================================================
#ifdef __cplusplus
typedef cv::cuda::GpuMat* GpuMat;
#else
typedef void* GpuMat;
#endif

GpuMat GpuMat_New();
void GpuMat_Close(GpuMat m);
void GpuMat_Upload(GpuMat gm, Mat m);
Mat GpuMat_ConvertTo(GpuMat m);
int GpuMat_Rows(GpuMat m);
int GpuMat_Cols(GpuMat m);
int GpuMat_Empty(GpuMat m);

// =============================================================================
//   HOG
// =============================================================================

#ifdef __cplusplus
typedef cv::Ptr<cv::cuda::HOG>* HOG;
#else
typedef void* HOG;
#endif

// Wrapper for std::vector<double>
typedef struct VecDouble {
    double* val;
    int length;
} VecDouble;

HOG HOG_Create();
HOG HOG_Create_WithParams(Size win_size, Size block_size, Size block_stride, Size cell_size, int nbins);
void HOG_Close(HOG h);
void HOG_SetSVMDetector(HOG h, Mat detector);
Rects HOG_DetectMultiScale(HOG h, GpuMat image);
Rects HOG_DetectMultiScale_WithConf(HOG h, GpuMat image, VecDouble* confidence);
void HOG_SetGammaCorrection(HOG h, bool gamma);
void HOG_SetGroupThreshold(HOG, int group_threshold);
void HOG_SetHitThreshold(HOG, double hit_threshold);
void HOG_SetL2hysThreshold(HOG, double l2hys_threshold);
void HOG_SetNumLevels(HOG, int num_levels);
void HOG_SetScaleFactor(HOG, double scale_factor);
void HOG_SetWinSigma(HOG, double win_sigma);
void HOG_SetWinStride(HOG, Size win_stride);

bool HOG_GetGammaCorrection(HOG);
int HOG_GetGroupThreshold(HOG);
double HOG_GetHitThreshold(HOG);
double HOG_GetL2hysThreshold(HOG);
int HOG_GetNumLevels(HOG);
double HOG_GetScaleFactor(HOG);
double HOG_GetWinSigma(HOG);
Size HOG_GetWinStride(HOG);

// =============================================================================
//   CascadeClassifier
// =============================================================================

#ifdef __cplusplus
typedef cv::Ptr<cv::cuda::CascadeClassifier>* GpuCascade;
#else
typedef void* GpuCascade;
#endif

GpuCascade GpuCascade_Create(const char* const filename);
void GpuCascade_Close(GpuCascade);
Rects GpuCascade_DetectMultiScale(GpuCascade, GpuMat);

void GpuCascade_SetFindLargestObject(GpuCascade, bool);
void GpuCascade_SetMaxNumObjects(GpuCascade, int);
void GpuCascade_SetMinNeighbors(GpuCascade, int);
void GpuCascade_SetMaxObjectSize(GpuCascade, Size);
void GpuCascade_SetMinObjectSize(GpuCascade, Size);
void GpuCascade_SetScaleFactor(GpuCascade, double);

Size GpuCascade_GetClassifierSize(GpuCascade);
bool GpuCascade_GetFindLargestObject(GpuCascade);
int GpuCascade_GetMaxNumObjects(GpuCascade);
int GpuCascade_GetMinNeighbors(GpuCascade);
Size GpuCascade_GetMaxObjectSize(GpuCascade);
Size GpuCascade_GetMinObjectSize(GpuCascade);
double GpuCascade_GetScaleFactor(GpuCascade);

#ifdef __cplusplus
}
#endif

#endif  // _OPENCV3_CUDA_H_
