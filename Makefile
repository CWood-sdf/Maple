CXX = clang
CXXFLAGS = -Wall -Wextra -pedantic -std=c++20 -ffunction-sections -fdata-sections -frtti -g
LDFLAGS = /libpath:"C:\Program Files\Microsoft Visual Studio\2022\Community\VC\Tools\Llvm"
HEADERS = $(wildcard Maple/*.h)

BUILD_DIR = Maple/out

SRC = $(wildcard Maple/*.cpp) 
OBJ = $(addprefix $(BUILD_DIR)/, $(addsuffix .obj, $(basename $(notdir $(SRC)))) )
EXEC = Maple/out/main.exe

all: $(EXEC)

$(EXEC): $(OBJ)
	$(CXX) $(CXXFLAGS) $(OBJ) -o $(EXEC)

$(BUILD_DIR)/%.obj: Maple/%.cpp $(HEADERS)
	@echo $<
	@$(CXX) $(CXXFLAGS) -c $< -o $@

clean:
	rm -rf $(OBJ) $(EXEC)

test:
	@echo $(notdir C:/Users/woodc/Documents/Maple/Maple.cpp)