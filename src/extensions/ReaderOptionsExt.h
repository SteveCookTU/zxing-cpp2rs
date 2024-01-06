/*
* Copyright 2016 Nu-book Inc.
* Copyright 2016 ZXing authors
* Copyright 2020 Axel Waggershauser
*/
// SPDX-License-Identifier: Apache-2.0

#pragma once

#include "BarcodeFormat.h"
#include "ReaderOptions.h"

namespace ZXing {

class ReaderOptionsExt : public ReaderOptions
{

public:
    ReaderOptionsExt() {}

    ReaderOptionsExt& addFormat(BarcodeFormat f)&
    {
        BarcodeFormats newFormat(f);
        newFormat |= this->formats();
        return (void)(setFormats(f)), *this;
    }

    ReaderOptionsExt&& addFormat(BarcodeFormat f) &&
    {
        BarcodeFormats newFormat(f);
        newFormat |= this->formats();
        return (void)(setFormats(newFormat)), std::move(*this);
    }

    ReaderOptions& asOptions()
    {
        return *this;
    }
};

} // ZXing
