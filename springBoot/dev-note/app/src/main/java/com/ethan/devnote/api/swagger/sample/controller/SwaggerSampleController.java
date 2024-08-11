package com.ethan.devnote.api.swagger.sample.controller;

import com.ethan.devnote.api.swagger.sample.domain.vo.SwaggerGetParamVO;
import com.ethan.devnote.api.swagger.sample.domain.vo.SwaggerRequestBodyVO;
import io.swagger.v3.oas.annotations.Operation;
import io.swagger.v3.oas.annotations.Parameter;
import io.swagger.v3.oas.annotations.media.Content;
import io.swagger.v3.oas.annotations.media.Schema;
import io.swagger.v3.oas.annotations.responses.ApiResponse;
import io.swagger.v3.oas.annotations.responses.ApiResponses;
import io.swagger.v3.oas.annotations.tags.Tag;
import org.springframework.http.MediaType;
import org.springframework.web.bind.annotation.*;

import static org.springframework.http.MediaType.*;

@RestController
@RequestMapping("/api/v1/swagger")
@Tag(name = "Swagger Sample ", description = "스웨거 개발 샘플 파일입니다.")
public class SwaggerSampleController {

    @Operation(summary = "기본API", description = "기본적으로 실행되는 API입니다.")
    @ApiResponses(value = @ApiResponse(responseCode = "200", description = "고정 값 success"))
    @GetMapping("/")
    public String getDefault() {
        return "success";
    }

    /*
    @Operation(summary = "쿼리 스트링 예제", description = "쿼리 스트링 예제입니다.")
    @ApiResponses(value = @ApiResponse(responseCode = "200", description = "고정 값 success"))
    @GetMapping("/query")
    public String getQueryString(SwaggerGetParamVO param) {
        return "success";
    }*/

    @Operation(summary = "쿼리 스트링 예제2", description = "쿼리 스트링 예제2 입니다.")
    @ApiResponses(value = @ApiResponse(responseCode = "200", description = "고정 값 success"))
    @GetMapping("/query2")
    public String getQueryString(
            @Parameter(description = "제목을 입력!", required = true) @RequestParam(required = true) String title,
            @Parameter(description = "내용을 입력!", required = false) @RequestParam(required = false) String desc                                  ) {
        return "success";
    }

    @Operation(summary = "Path Variable 예제", description = "Path Variable 예제")
    @ApiResponses(value = @ApiResponse(responseCode = "200", description = "고정 값 success"))
    @GetMapping("/{id}")
    public String getPathVariable(
            @Parameter(description = "식별 아이디", required = true) @PathVariable("id") String id
    ){
        return id;
    }


    @Operation(summary = "Header 예제", description = "Header 예제")
    @ApiResponses(value = @ApiResponse(responseCode = "200", description = "고정 값 success"))
    @GetMapping("/head")
    public String getHeader(
            @RequestHeader(value = "token") String token
    ){
        return token;
    }


    @Operation(summary = "Header,Path Variable, QueryString 혼합 예제", description = "Header,Path Variable, QueryString 혼합 예제입니다")
    @ApiResponses(value = @ApiResponse(responseCode = "200", description = "고정 값 success"))
    @GetMapping("/combination/{id}")
    public String getHeader(
            @RequestHeader(value = "token") String token,
            @Parameter(description = "식별 아이디", required = true) @PathVariable("id") String id,
            @Parameter(description = "제목을 입력!", required = true) @RequestParam(required = true) String title,
            @Parameter(description = "내용을 입력!", required = false) @RequestParam(required = false) String desc

    ){
        return token;
    }

    @Operation(summary = "Post 예제", description = "Post 예제입니다.")
    @ApiResponses(value = @ApiResponse(responseCode = "200", description = "고정 값 success"))
    @PostMapping(value="/", produces = MediaType.APPLICATION_JSON_VALUE)
    public SwaggerRequestBodyVO post(
            @io.swagger.v3.oas.annotations.parameters.RequestBody(description = "Request Body 예제")
            @RequestBody SwaggerRequestBodyVO body
    ){
        return body;
    }
}
