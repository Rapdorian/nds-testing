
//{{BLOCK(wood)

//======================================================================
//
//	wood, 256x256@8, 
//	+ palette 256 entries, not compressed
//	+ 257 tiles (t|f reduced) not compressed
//	+ regular map (flat), not compressed, 32x32 
//	Total size: 512 + 16448 + 2048 = 19008
//
//	Time-stamp: 2019-09-08, 23:44:33
//	Exported by Cearn's GBA Image Transmogrifier, v0.8.15
//	( http://www.coranac.com/projects/#grit )
//
//======================================================================

#ifndef GRIT_WOOD_H
#define GRIT_WOOD_H

#define woodTilesLen 16448
extern const unsigned int woodTiles[4112];

#define woodMapLen 2048
extern const unsigned short woodMap[1024];

#define woodPalLen 512
extern const unsigned short woodPal[256];

#endif // GRIT_WOOD_H

//}}BLOCK(wood)
