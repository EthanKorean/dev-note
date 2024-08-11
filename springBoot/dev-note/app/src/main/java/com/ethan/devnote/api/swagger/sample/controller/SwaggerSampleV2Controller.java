package com.ethan.devnote.api.swagger.sample.controller;

import io.swagger.v3.oas.annotations.Operation;
import io.swagger.v3.oas.annotations.responses.ApiResponse;
import io.swagger.v3.oas.annotations.responses.ApiResponses;
import io.swagger.v3.oas.annotations.tags.Tag;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RestController;

@RestController
@RequestMapping("/api/v2/swagger")
@Tag(name = "Swagger Sample V2 ", description = "스웨거 개발 샘플 파일입니다.")
public class SwaggerSampleV2Controller {

    @Operation(summary = "기본 버전 2 API", description = "기본적으로 실행되는 API입니다.")
    @ApiResponses(value = @ApiResponse(responseCode = "200", description = "고정 값 success"))
    @GetMapping("/")
    public String getDefault() {
        return "success";
    }
}
