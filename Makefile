
CXX = clang
CXXFLAGS = -Wall -Wextra -Wpedantic  -std=c++2b
SRC_DIR = Maple
INC_DIR = Maple
LD_FLAGS = 
TOUCH_FILE =
HEADERS = $(wildcard Maple/*.h Maple/*/*.h)
CLEAN = 
ifeq ($(RELEASE), 1)
	CXXFLAGS += -O3  -ffunction-sections -fdata-sections -frtti 
else 
	CXXFLAGS += -glldb
endif
ifeq ($(OS), Windows_NT)
	CXXFLAGS += -DWIN32
	OBJ_EXT = obj
	BUILD_DIR = ./bin/win64
	EXEC = $(BUILD_DIR)/main.exe
	MKDIR = md "$(@D)" 2> nul || :
	CLEAN = powershell "Get-ChildItem -Path $(BUILD_DIR) -Include *.obj, *.exe -Recurse -Force | Remove-Item -Force "
else
	CXXFLAGS += -DUNIX
	LD_FLAGS += -lstdc++ -lm
	OBJ_EXT = o
	BUILD_DIR = bin/linux64
	EXEC = $(BUILD_DIR)/main
	CXX = clang-15
	MKDIR = mkdir -p $(dir $@)
	CLEAN = rm -rf $(BUILD_DIR)/*.o $(BUILD_DIR)/* $(BUILD_DIR)/*.out 
endif


SRC = $(wildcard $(SRC_DIR)/*.cpp $(SRC_DIR)/*/*.cpp) 
OBJ = $(addprefix $(BUILD_DIR)/, $(addsuffix .$(OBJ_EXT), $(basename $(SRC)))) 


all: $(EXEC)

$(EXEC): $(OBJ)
	$(CXX) $(CXXFLAGS) $(OBJ) -o $(EXEC) $(LD_FLAGS)

$(BUILD_DIR)/%.$(OBJ_EXT): %.cpp $(HEADERS) Makefile
	@$(MKDIR)
	@echo $<
	@$(CXX) -c $< $(CXXFLAGS) -o $@

clean:
	$(CLEAN)

compile_objects: $(OBJ)

run: $(EXEC) $(OBJ) $(HEADERS)
	$(EXEC)

test:
	@echo $(notdir C:/Users/woodc/Documents/Maple/Maple.cpp)
	@echo $(OS)
	@echo $(BUILD_DIR)
