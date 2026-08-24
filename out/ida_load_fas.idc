// Original author Personal BLOG: http://blog.sina.com.cn/southdy
//  Edited to support all segments by M.A.
#include <idc.idc>

#define VERBOSE 1
// Previous/initial version was handling only the first segment. Now line 110 is commentted out.
// Uncomment line 110 if you wish it again handle only first segment.

static main(void)
{
  auto pszFileName;
  auto hFile;

  auto dwPosSymTbl;
  auto dwSizeSymTbl;

  auto dwPosSym;

  auto dwPosSrcTbl;
  auto dwSizeSrcTbl;

  auto dwPosSrc;

  auto dwSymEA;
  auto dwSymEA64;
  auto wdSymFlag;
  auto btSymSize;
  auto btSymType;

  auto btSrcType;
  auto btSrcSize;

  auto dwIndex;

  auto dwSegStartEA;
  auto dwSegEndEA;

  auto btNameData;
  auto dwNameType;

  pszFileName = AskFile(0, "*.fas", "Choose the symbol-file");
  if ("" == pszFileName) {
    //Message("load_fas: File not chhosen\n");
    return;
  }
  
  hFile = fopen(pszFileName, "rb");
  if (0 == hFile) { 
    Message("load_fas: File '%s' can't be opened.\n", pszFileName);
    return;
  }
  
  dwSegStartEA = FirstSeg();
  /*
  auto eaSegStart= FirstSeg();
  while(eaSegStart!=BADADDR){
  	auto segEnd= SegEnd(eaSegStart);
	auto segName= SegName(eaSegStart);
	auto segType= GetSegmentAttr(eaSegStart,SEGATTR_TYPE);
	Message("segment '%s' startEA=%x, %x, %x\n", segName, eaSegStart, segEnd, segType);
    eaSegStart = NextSeg(eaSegStart+1);	
  }
 long GetSegmentAttr(eaSegStart,SEGATTR_TYPE); SEGATTR_COMB combination  SEGATTR_PERM permissions
  SEGATTR_BITNESS bitness(0: 16, 1: 32, 2: 64 bit segment)  SEGATTR_FLAGS segment flags
  */
  
  dwSegEndEA = SegEnd(dwSegStartEA);
  if (VERBOSE) {
	Message("load_fas: First segment: %x-%x\n", dwSegStartEA, dwSegEndEA);
  }
  
  // read SymTbl and SrcTbl, position and size: 
  fseek(hFile, 24, 0);
  dwPosSymTbl = readlong(hFile, 0);
  dwSizeSymTbl = readlong(hFile, 0);
  dwPosSrcTbl = readlong(hFile, 0);
  dwSizeSrcTbl = readlong(hFile, 0);
  // printout them:
  if (VERBOSE) {
	Message("load_fas: posSymTbl,sizeSymTbl,posSrcTbl,sizeSrcTbl: %x-%x-%x-%x\n", dwPosSymTbl, dwSizeSymTbl, dwPosSrcTbl, dwSizeSrcTbl);
  }
  
  // Iterate over symbols in symbol table of .fas file: 
  for (dwPosSym = 0; dwPosSym < dwSizeSymTbl; dwPosSym = dwPosSym + 32)
  {
	auto pszSymName;
	
    // read 
	fseek(hFile, dwPosSymTbl + dwPosSym, 0);
	dwSymEA   = readlong(hFile, 0);
	dwSymEA64 = readlong(hFile, 0);
	wdSymFlag = readshort(hFile, 0);
	btSymSize = fgetc(hFile);
	btSymType = fgetc(hFile);
	
    // read 
	fseek(hFile, dwPosSymTbl + dwPosSym + 28, 0);
	dwPosSrc = readlong(hFile, 0);
	
    // read btSrcType, btSymType
	fseek(hFile, dwPosSrcTbl + dwPosSrc + 16, 0);
	btSrcType = fgetc(hFile);
	btSrcSize = fgetc(hFile);

	//filter out ""wrong"" symbols, and rename the rest:
	if (   0x1A == btSrcType && 0 != btSymType 
	     //`{if (bYouNeedHandleOnlyFirstSegment) "CommandToProgrammer: manually comment out the next line";}
		 // && dwSegStartEA <= dwSymEA && dwSymEA < dwSegEndEA 
	   )
	{
	  pszSymName = "";
	  dwNameType = SN_CHECK;
	  
	  for (dwIndex = 0; dwIndex < btSrcSize; dwIndex = dwIndex + 1)
	  {
		btNameData = fgetc(hFile);

		if ((0 == dwIndex) && ('.' == btNameData)) {
		  dwNameType = SN_LOCAL;
		}

		pszSymName = pszSymName + btNameData;
	  }
	  
      // Avoid "protected" names: rename the identifier, prefix it with '_'   
	  if (    0 == strstr(pszSymName, "sub") 
	       || 0 == strstr(pszSymName, "byte") || 0 == strstr(pszSymName, "word") 
	       || 0 == strstr(pszSymName, "dword") || 0 == strstr(pszSymName, "qword") 
		 )
	  {
		pszSymName = "_" + pszSymName;
	  }
	  // Rename or give new name to original name to extracted name:
	  MakeNameEx(dwSymEA, pszSymName, dwNameType);

	  if (0 && VERBOSE) {
		Message("%x-%x-%d-%d-%d-%s\n", dwSymEA, wdSymFlag, btSymType, btSymSize, btSrcSize, pszSymName);
	  }
	}
  }

  Message("load_fas: Symbol file %s has been loaded successfully.\n", pszFileName);

  fclose(hFile);
}

