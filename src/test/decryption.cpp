// #include <stdint.h>
#include <inttypes.h>
#include <vector>
#include <stdio.h>

std::vector<uint32_t>* RomBios = new std::vector<uint32_t>(0x20000);
std::vector<uint32_t>* RomGame = new std::vector<uint32_t>(0x1000000);
std::vector<uint32_t>* RomGame_D = new std::vector<uint32_t>(0x1000000);

uint32_t cps3_key1 = 0xb5fe053e;
uint32_t cps3_key2 = 0xfc03925a;


// code lifted from FBA cps3init.cpp (of which the decryption code
// was lifted from Andreas Naive's blog)
static uint16_t rotate_left(uint16_t value, int32_t n)
{
   int32_t aux = value>>(16-n);
   return ((value<<n)|aux) & 0xffff;
}

static uint16_t rotxor(uint16_t val, uint16_t x)
{
	uint16_t res;
	res = val + rotate_left(val,2);
	res = rotate_left(res,4) ^ (res & (val ^ x));
	return res;
}

static uint32_t cps3_mask(uint32_t address, uint32_t key1, uint32_t key2)
{
	uint16_t val;
	address ^= key1;
	val = (address & 0xffff) ^ 0xffff;
	val = rotxor(val, key2 & 0xffff);
	val ^= (address >> 16) ^ 0xffff;
	val = rotxor(val, key2 >> 16);
	val ^= (address & 0xffff) ^ (key2 & 0xffff);
	return val | (val << 16);
}

static void cps3_decrypt_bios(void)
{
	uint32_t * coderegion = (uint32_t *)RomBios;
	for (int32_t i=0; i<0x20000; i+=4) {
		uint32_t xormask = cps3_mask(i, cps3_key1, cps3_key2);
		/* a bit of a hack, don't decrypt the FLASH commands which are transfered by SH2 DMA */
		if ( (i<0x1ff00) || (i>0x1ff6b) )
			coderegion[i/4] ^= xormask;
	}
}

static void cps3_decrypt_game(void)
{
	uint32_t * coderegion = (uint32_t *)RomGame;
	uint32_t * decrypt_coderegion = (uint32_t *)RomGame_D;
	
	for (int32_t i=0; i<0x1000000; i+=4) {
		uint32_t xormask = cps3_mask(i + 0x06000000, cps3_key1, cps3_key2);
		decrypt_coderegion[i/4] = coderegion[i/4] ^ xormask;
	}
}

int main(int argc, char** argv) {
  uint16_t rot_xor = rotxor(0xab04, 0x98fe);
  printf("rotxor is %" PRIu16 "\n", rot_xor);
  uint32_t xormask = cps3_mask(0, cps3_key1, cps3_key2);
  printf("xormask is %" PRIu32 "\n", xormask);
  return 0;
}
