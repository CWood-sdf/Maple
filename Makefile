
CXX = clang
CXXFLAGS = -Wall -Wextra -Wpedantic -std=c++2b -ffunction-sections -fdata-sections -frtti
LD_FLAGS = 
TOUCH_FILE =
HEADERS = $(wildcard Maple/*.h)
ifeq ($(RELEASE), 1)
	CXXFLAGS += -O3
else 
	CXXFLAGS += -g
endif
ifeq ($(OS), Windows_NT)
	CXXFLAGS += -DWIN32
	OBJ_EXT = obj
	BUILD_DIR = ./bin/win64
	EXEC = $(BUILD_DIR)/main.exe
else
	CXXFLAGS += -DUNIX -gdwarf-4
	LD_FLAGS += -lstdc++ -lm
	OBJ_EXT = o
	BUILD_DIR = ./bin/linux64
	EXEC = $(BUILD_DIR)/main
	CXX = clang-15
endif


SRC = $(wildcard Maple/*.cpp) 
OBJ = $(addprefix $(BUILD_DIR)/, $(addsuffix .$(OBJ_EXT), $(basename $(notdir $(SRC)))) )


all: $(EXEC)

$(EXEC): $(OBJ)
	$(CXX) $(CXXFLAGS) $(OBJ) -o $(EXEC) $(LD_FLAGS)

$(BUILD_DIR)/%.$(OBJ_EXT): Maple/%.cpp $(HEADERS)
	@echo $<
	@$(CXX) $(CXXFLAGS) -c $< -o $@

clean:
	rm -rf $(OBJ) $(EXEC)

compile_objects: $(OBJ)

run: $(EXEC) $(OBJ) $(HEADERS)
	$(EXEC)

test:
	@echo $(notdir C:/Users/woodc/Documents/Maple/Maple.cpp)
	@echo $(OS)
	@echo $(BUILD_DIR)
